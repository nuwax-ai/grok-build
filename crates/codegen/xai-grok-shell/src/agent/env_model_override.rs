//! Build a [`ConfigModelOverride`] from `GROK_MODEL_*` environment variables.
//!
//! This module is the bridge between the env-var constants in `xai_grok_models::env_models`
//! and the shell's [`ConfigModelOverride`] type. It is called from `config.rs` during
//! `new_from_toml_cfg` to inject a user-defined model defined purely through env vars.
//!
//! Parsing is delegated to [`xai_grok_models::read_env_model_definition`] to avoid
//! duplicating env-var extraction logic.
//!
//! # Model ID resolution
//!
//! When `GROK_MODEL_ID` is not explicitly set, the builder falls back to
//! `GROK_DEFAULT_MODEL` — but only when the user has also provided enough
//! model-definition context (`GROK_MODEL_BASE_URL`, `GROK_MODEL_API_KEY`, or
//! `GROK_MODEL_ENV_KEY`). This lets you use a minimal two-variable setup:
//!
//! ```text
//! GROK_DEFAULT_MODEL=deepseek-chat
//! GROK_MODEL_BASE_URL=https://api.deepseek.com/v1
//! GROK_MODEL_API_KEY=sk-xxx
//! ```
//!
//! # Display name resolution
//!
//! `GROK_MODEL_DISPLAY_NAME` > `GROK_MODEL_NAME` > `model_id`

use crate::agent::config::{ConfigModelOverride, EnvKeys};
use crate::sampling::ApiBackend;

/// Read all `GROK_MODEL_*` environment variables and, when a model identity can
/// be determined, return a `(key, ConfigModelOverride)` pair that can be
/// inserted into `config_models` during config loading.
///
/// Returns `None` when neither `GROK_MODEL_ID` nor a usable
/// `GROK_DEFAULT_MODEL`-with-definition combination is available.
pub fn build_env_model_override() -> Option<(String, ConfigModelOverride)> {
    let def = xai_grok_models::read_env_model_definition();

    // ── Determine the model_id (catalog key) ──────────────────────────
    let model_id = def.model_id.or_else(|| {
        // Fallback: derive model_id from GROK_DEFAULT_MODEL when the user
        // has also provided model-definition vars (base_url, api_key, etc.)
        let default_model = std::env::var(xai_grok_models::ENV_DEFAULT_MODEL).ok()?;
        if default_model.trim().is_empty() {
            return None;
        }
        // Only promote when there's enough context to define a real model.
        if def.base_url.is_none() && def.api_key.is_none() && def.env_key.is_none() {
            return None;
        }
        tracing::info!(
            default_model = % default_model.trim(),
            "GROK_MODEL_ID not set; deriving model_id from GROK_DEFAULT_MODEL"
        );
        Some(default_model.trim().to_owned())
    })?;

    // model_name is used as both the routing slug (API model field) and
    // as a fallback for display_name below. We clone the inner String for
    // the routing slug because the outer Option is moved into .or() later.
    let model_name = def.model_name.clone();

    // ── API backend ───────────────────────────────────────────────────
    let api_backend = def.api_backend.as_deref().and_then(|v| {
        v.parse::<ApiBackend>().ok().or_else(|| {
            tracing::warn!(
                "GROK_MODEL_API_BACKEND: unrecognized value '{}', \
                 expected chat_completions | responses | messages",
                v
            );
            None
        })
    });

    let extra_headers: indexmap::IndexMap<String, String> = def
        .extra_headers
        .map(|h| h.into_iter().collect())
        .unwrap_or_default();

    // ── Routing slug (API model field) ────────────────────────────────
    // GROK_MODEL_NAME > model_id
    let routing_slug = model_name.clone().unwrap_or_else(|| model_id.clone());

    // ── Display name (UI picker) ──────────────────────────────────────
    // GROK_MODEL_DISPLAY_NAME > GROK_MODEL_NAME > model_id
    let display_name = def
        .display_name
        .or(model_name)
        .unwrap_or_else(|| model_id.clone());

    let override_ = ConfigModelOverride {
        model: Some(routing_slug),
        base_url: def.base_url,
        name: Some(display_name),
        description: None,
        api_key: def.api_key,
        env_key: def.env_key.map(EnvKeys::single),
        auth_provider: None,
        model_provider: None,
        api_base_url: None,
        max_completion_tokens: def.max_tokens,
        temperature: def.temperature,
        top_p: def.top_p,
        api_backend,
        extra_headers,
        context_window: def.context_window,
        auto_compact_threshold_percent: None,
        system_prompt_label: None,
        use_concise: None,
        agent_type: def.agent_type,
        inference_idle_timeout_secs: None,
        max_retries: def.max_retries,
        hidden: Some(false),
        supported_in_api: Some(true),
        reasoning_effort: None,
        supports_reasoning_effort: None,
        reasoning_efforts: Vec::new(),
        supports_backend_search: None,
        compactions_remaining: None,
        compaction_at_tokens: None,
        show_model_fingerprint: None,
        stream_tool_calls: def.stream_tool_calls,
    };

    // ── Safety net: warn when base_url is missing ─────────────────────
    if override_.base_url.is_none() {
        tracing::warn!(
            model_id = % model_id,
            "GROK_MODEL_BASE_URL is not set; model '{}' will fall back to \
             the default xAI inference endpoint. Set GROK_MODEL_BASE_URL to \
             your custom endpoint URL.",
            model_id
        );
    }

    tracing::info!(
        model_id = % model_id,
        base_url = ? override_.base_url,
        api_backend = ? override_.api_backend,
        "env model override: built from GROK_MODEL_* environment variables"
    );

    Some((model_id, override_))
}
