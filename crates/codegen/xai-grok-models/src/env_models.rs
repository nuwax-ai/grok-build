//! Environment-variable driven model configuration.
//!
//! Provides constants and helpers for overriding model selection and defining
//! complete custom models via environment variables. Designed to be imported
//! as `pub use` from the crate root so callers see `xai_grok_models::ENV_MODEL_ID`, etc.
//!
//! # Two tiers of environment variables
//!
//! ## Tier 1 — Default model selection (`GROK_DEFAULT_*`)
//!
//! These change the **compiled-in fallback** (lowest priority) for model roles:
//!
//! | Variable | Role |
//! |---|---|
//! | `GROK_DEFAULT_MODEL` | Primary coding / general fallback |
//! | `GROK_DEFAULT_WEB_SEARCH_MODEL` | Web search tool synthesis |
//! | `GROK_DEFAULT_IMAGE_DESCRIPTION_MODEL` | Image describe |
//! | `GROK_DEFAULT_SESSION_SUMMARY_MODEL` | Session title generation |
//!
//! Priority chain (e.g. for web search):
//! `CLI flag > GROK_WEB_SEARCH_MODEL > config.toml > remote > GROK_DEFAULT_WEB_SEARCH_MODEL`
//!
//! When `GROK_MODEL_ID` is **not** set but `GROK_DEFAULT_MODEL` is set together with
//! model-definition vars (`GROK_MODEL_BASE_URL`, `GROK_MODEL_API_KEY`, etc.), the
//! default model name is automatically promoted to serve as `GROK_MODEL_ID`.
//!
//! ## Tier 2 — Custom model definition (`GROK_MODEL_*`)
//!
//! These define a **complete custom model** that is injected into the model catalog
//! at the highest priority (can't be overridden by config.toml):
//!
//! | Variable | Description |
//! |---|---|
//! | `GROK_MODEL_ID` | Catalog key (required) |
//! | `GROK_MODEL_NAME` | API routing slug — sent as the `model` field in requests |
//! | `GROK_MODEL_DISPLAY_NAME` | Human-readable name shown in the UI picker |
//! | `GROK_MODEL_BASE_URL` | Inference endpoint base URL |
//! | `GROK_MODEL_API_KEY` | Static API key |
//! | `GROK_MODEL_ENV_KEY` | Env var name that holds the API key at runtime |
//! | `GROK_MODEL_API_BACKEND` | Protocol: `chat_completions` / `responses` / `messages` |
//! | `GROK_MODEL_CONTEXT_WINDOW` | Context window size in tokens |
//! | `GROK_MODEL_EXTRA_HEADERS` | JSON map of extra HTTP headers |
//! | `GROK_MODEL_AUTH_SCHEME` | (reserved — not yet wired) |
//! | `GROK_MODEL_TEMPERATURE` | Sampling temperature |
//! | `GROK_MODEL_TOP_P` | Nucleus sampling top-p |
//! | `GROK_MODEL_MAX_TOKENS` | Max completion tokens |
//! | `GROK_MODEL_AGENT_TYPE` | Agent type label |
//! | `GROK_MODEL_STREAM_TOOL_CALLS` | `true` / `false` |
//! | `GROK_MODEL_MAX_RETRIES` | Max retry count |

use std::collections::HashMap;
use std::sync::LazyLock;

// ── Default model ID overrides (Tier 1) ───────────────────────────────────

pub const ENV_DEFAULT_MODEL: &str = "GROK_DEFAULT_MODEL";
pub const ENV_DEFAULT_WEB_SEARCH_MODEL: &str = "GROK_DEFAULT_WEB_SEARCH_MODEL";
pub const ENV_DEFAULT_IMAGE_DESCRIPTION_MODEL: &str = "GROK_DEFAULT_IMAGE_DESCRIPTION_MODEL";
pub const ENV_DEFAULT_SESSION_SUMMARY_MODEL: &str = "GROK_DEFAULT_SESSION_SUMMARY_MODEL";

/// Read and trim an environment variable, returning `None` when the
/// variable is unset or whitespace-only. Used both by the one-shot
/// [`LazyLock`] caches below and by [`read_env_model_definition`].
fn trimmed_env(key: &str) -> Option<String> {
    let v = std::env::var(key).ok()?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

static ENV_DEFAULT_MODEL_CACHE: LazyLock<Option<String>> =
    LazyLock::new(|| trimmed_env(ENV_DEFAULT_MODEL));
static ENV_DEFAULT_WEB_SEARCH_CACHE: LazyLock<Option<String>> =
    LazyLock::new(|| trimmed_env(ENV_DEFAULT_WEB_SEARCH_MODEL));
static ENV_DEFAULT_IMAGE_DESCRIPTION_CACHE: LazyLock<Option<String>> =
    LazyLock::new(|| trimmed_env(ENV_DEFAULT_IMAGE_DESCRIPTION_MODEL));
static ENV_DEFAULT_SESSION_SUMMARY_CACHE: LazyLock<Option<String>> =
    LazyLock::new(|| trimmed_env(ENV_DEFAULT_SESSION_SUMMARY_MODEL));

pub fn env_default_model() -> Option<&'static str> {
    ENV_DEFAULT_MODEL_CACHE.as_deref()
}
pub fn env_default_web_search() -> Option<&'static str> {
    ENV_DEFAULT_WEB_SEARCH_CACHE.as_deref()
}
pub fn env_default_image_description() -> Option<&'static str> {
    ENV_DEFAULT_IMAGE_DESCRIPTION_CACHE.as_deref()
}
pub fn env_default_session_summary() -> Option<&'static str> {
    ENV_DEFAULT_SESSION_SUMMARY_CACHE.as_deref()
}

// ── Custom model definition (Tier 2: GROK_MODEL_*) ───────────────────────

pub const ENV_MODEL_ID: &str = "GROK_MODEL_ID";
/// API routing slug — sent as the `model` field in inference requests.
/// Falls back to `GROK_MODEL_ID` when unset.
pub const ENV_MODEL_NAME: &str = "GROK_MODEL_NAME";
/// Human-readable display name shown in the UI model picker.
/// Falls back to `GROK_MODEL_NAME`, then `GROK_MODEL_ID`.
pub const ENV_MODEL_DISPLAY_NAME: &str = "GROK_MODEL_DISPLAY_NAME";
pub const ENV_MODEL_BASE_URL: &str = "GROK_MODEL_BASE_URL";
pub const ENV_MODEL_API_KEY: &str = "GROK_MODEL_API_KEY";
pub const ENV_MODEL_ENV_KEY: &str = "GROK_MODEL_ENV_KEY";
pub const ENV_MODEL_API_BACKEND: &str = "GROK_MODEL_API_BACKEND";
pub const ENV_MODEL_CONTEXT_WINDOW: &str = "GROK_MODEL_CONTEXT_WINDOW";
pub const ENV_MODEL_EXTRA_HEADERS: &str = "GROK_MODEL_EXTRA_HEADERS";
/// Reserved for future use — not yet wired through to `ConfigModelOverride` / `ModelInfo`.
pub const ENV_MODEL_AUTH_SCHEME: &str = "GROK_MODEL_AUTH_SCHEME";
pub const ENV_MODEL_TEMPERATURE: &str = "GROK_MODEL_TEMPERATURE";
pub const ENV_MODEL_TOP_P: &str = "GROK_MODEL_TOP_P";
pub const ENV_MODEL_MAX_TOKENS: &str = "GROK_MODEL_MAX_TOKENS";
pub const ENV_MODEL_AGENT_TYPE: &str = "GROK_MODEL_AGENT_TYPE";
pub const ENV_MODEL_STREAM_TOOL_CALLS: &str = "GROK_MODEL_STREAM_TOOL_CALLS";
pub const ENV_MODEL_MAX_RETRIES: &str = "GROK_MODEL_MAX_RETRIES";

/// Structured data parsed from `GROK_MODEL_*` environment variables.
#[derive(Debug, Clone, Default)]
pub struct EnvModelDefinition {
    pub model_id: Option<String>,
    /// API routing slug.
    pub model_name: Option<String>,
    /// Human-readable UI display name.
    pub display_name: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub env_key: Option<String>,
    pub api_backend: Option<String>,
    pub context_window: Option<u64>,
    pub extra_headers: Option<HashMap<String, String>>,
    /// Reserved for future use — not yet wired through to `ConfigModelOverride` / `ModelInfo`.
    pub auth_scheme: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<u32>,
    pub agent_type: Option<String>,
    pub stream_tool_calls: Option<bool>,
    pub max_retries: Option<u32>,
}

/// Parse all `GROK_MODEL_*` environment variables into an [`EnvModelDefinition`].
///
/// All string values are trimmed of surrounding whitespace; whitespace-only
/// values are treated as `None`.
pub fn read_env_model_definition() -> EnvModelDefinition {
    EnvModelDefinition {
        model_id: trimmed_env(ENV_MODEL_ID),
        model_name: trimmed_env(ENV_MODEL_NAME),
        display_name: trimmed_env(ENV_MODEL_DISPLAY_NAME),
        base_url: trimmed_env(ENV_MODEL_BASE_URL),
        api_key: trimmed_env(ENV_MODEL_API_KEY),
        env_key: trimmed_env(ENV_MODEL_ENV_KEY),
        api_backend: trimmed_env(ENV_MODEL_API_BACKEND),
        context_window: trimmed_env(ENV_MODEL_CONTEXT_WINDOW).and_then(|v| v.parse().ok()),
        extra_headers: trimmed_env(ENV_MODEL_EXTRA_HEADERS)
            .and_then(|v| serde_json::from_str::<HashMap<String, String>>(&v).ok())
            .filter(|m| !m.is_empty()),
        auth_scheme: trimmed_env(ENV_MODEL_AUTH_SCHEME),
        temperature: trimmed_env(ENV_MODEL_TEMPERATURE).and_then(|v| v.parse().ok()),
        top_p: trimmed_env(ENV_MODEL_TOP_P).and_then(|v| v.parse().ok()),
        max_tokens: trimmed_env(ENV_MODEL_MAX_TOKENS).and_then(|v| v.parse().ok()),
        agent_type: trimmed_env(ENV_MODEL_AGENT_TYPE),
        stream_tool_calls: trimmed_env(ENV_MODEL_STREAM_TOOL_CALLS).and_then(|v| {
            match v.to_ascii_lowercase().as_str() {
                "true" | "1" | "yes" => Some(true),
                "false" | "0" | "no" => Some(false),
                _ => None,
            }
        }),
        max_retries: trimmed_env(ENV_MODEL_MAX_RETRIES).and_then(|v| v.parse().ok()),
    }
}
