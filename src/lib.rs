//! hace-brain-runtime — Zeus CE Coordinator
//!
//! Routes BrainKernel calls to the correct CE backend.
//! Zeus only calls BrainKernel / BrainRuntime traits — never knows
//! Candle, ONNX, or llama.cpp internals.
//!
//! CE dispatch table:
//!   classify_complexity() -> Complexity
//!     Simple  -> CE.Algo   (AlgoParticle, deterministic, zero tokens)
//!     Medium  -> CE.Local  (HacedleBrain or LlamaBrain by config)
//!     Complex -> CE.Remote (cloud RACEX) or CE.Local fallback
//!
//! Hook-points registered (all pass-through in E4):
//!   hok://brain/before_route
//!   hok://brain/after_route
//!   hok://brain/before_execute
//!   hok://brain/after_execute

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use hace_brain_base::{
    BrainKernel, BrainError, BrainMode, Complexity,
    AlgoParticle, ReasonCtx, ReasonResult, classify_complexity,
};

pub use hace_brain_base::{
    BrainRuntime, BrainArtifact, ArtifactFormat, Embedding,
    BrainProfile, MemoryItem,
};

pub mod bridges;
pub mod prompt;

// ── RuntimeSio — runtime input/output structs ────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct RuntimeSio {
    pub prompt: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub provider: String,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeOutcome {
    pub status: String,
    pub result: Vec<u32>,
    pub telemetry: Telemetry,
}

#[derive(Debug, Clone, Default)]
pub struct Telemetry {
    pub latency_ms: u64,
    pub tokens_used: u32,
}

// ── ZeusRouter — selects CE based on complexity + mode ───────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeusConfig {
    pub mode:          BrainMode,
    pub max_local_ctx: usize,  // tokens — above this, force remote
    pub prefer_local:  bool,   // prefer local CE if available
}

impl Default for ZeusConfig {
    fn default() -> Self {
        Self { mode: BrainMode::Hybrid, max_local_ctx: 4096, prefer_local: true }
    }
}

pub struct ZeusRouter {
    config: ZeusConfig,
    algo:   AlgoParticle,
    // local_ce / remote_ce injected at runtime via register_*()
    local_ce:  Option<Box<dyn BrainKernel>>,
    remote_ce: Option<Box<dyn BrainKernel>>,
}

impl ZeusRouter {
    pub fn new(config: ZeusConfig) -> Self {
        Self { config, algo: AlgoParticle::new(), local_ce: None, remote_ce: None }
    }

    pub fn with_algo_rule(&mut self, pattern: &str, output: serde_json::Value) {
        self.algo.add_rule(pattern, output);
    }

    /// Register local CE (CE.Hacedle or CE.Llama)
    pub fn register_local(&mut self, ce: Box<dyn BrainKernel>) {
        self.local_ce = Some(ce);
    }

    /// Register remote CE (CE.Remote via RACEX)
    pub fn register_remote(&mut self, ce: Box<dyn BrainKernel>) {
        self.remote_ce = Some(ce);
    }

    /// Route and execute — the Zeus main dispatch path
    pub async fn route_and_execute(&self, ctx: ReasonCtx) -> Result<RouteResult, BrainError> {
        // Hook: before_route (E4: pass-through)
        let hook_id = "hok://brain/before_route";
        let _ = hook_id;  // E4: no-op, wire E5 governance here

        let complexity = classify_complexity(&ctx.action, payload_size(&ctx));

        let ce_id = self.select_ce(complexity);

        // Hook: after_route (E4: pass-through)
        let _ = "hok://brain/after_route";

        // Hook: before_execute
        let _ = "hok://brain/before_execute";

        let result = match ce_id {
            CeId::Algo => {
                self.algo.reason(ctx).await?
            }
            CeId::Local => {
                match &self.local_ce {
                    Some(ce) => ce.reason(ctx).await?,
                    None     => self.algo.reason(ctx).await?, // graceful fallback
                }
            }
            CeId::Remote => {
                match &self.remote_ce {
                    Some(ce) => ce.reason(ctx).await?,
                    None     => match &self.local_ce {
                        Some(lce) => lce.reason(ctx).await?,  // local fallback
                        None      => self.algo.reason(ctx).await?,
                    }
                }
            }
        };

        // Hook: after_execute (E4: pass-through; E5: evidence bundle)
        let _ = "hok://brain/after_execute";

        Ok(RouteResult { ce_id, result })
    }

    fn select_ce(&self, complexity: Complexity) -> CeId {
        match self.config.mode {
            BrainMode::AlgoOnly  => CeId::Algo,
            BrainMode::LocalLlm  => match complexity {
                Complexity::Simple => CeId::Algo,
                _                  => CeId::Local,
            },
            BrainMode::RemoteLlm => match complexity {
                Complexity::Simple => CeId::Algo,
                _                  => CeId::Remote,
            },
            BrainMode::Hybrid    => match complexity {
                Complexity::Simple  => CeId::Algo,
                Complexity::Medium  => CeId::Local,
                Complexity::Complex => if self.config.prefer_local { CeId::Local } else { CeId::Remote },
            },
        }
    }
}

fn payload_size(ctx: &ReasonCtx) -> usize {
    ctx.payload.to_string().len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeId {
    Algo,
    Local,   // CE.Hacedle | CE.Llama
    Remote,  // CE.Remote via RACEX
}

#[derive(Debug)]
pub struct RouteResult {
    pub ce_id:  CeId,
    pub result: ReasonResult,
}

// ── ZeusRuntime — full Zeus E4 runtime (router + CEs wired) ─────────────────

pub struct ZeusRuntime {
    pub router: ZeusRouter,
    pub profile: Option<BrainProfile>,
}

impl ZeusRuntime {
    pub fn new(config: ZeusConfig) -> Self {
        Self { router: ZeusRouter::new(config), profile: None }
    }

    pub fn with_profile(mut self, p: BrainProfile) -> Self {
        self.profile = Some(p);
        self
    }

    pub fn with_hacedle(mut self, model_path: &str) -> Self {
        #[cfg(feature = "hacedle")]
        {
            let mut brain = hace_fem_hacedle::HacedleBrain::new();
            let _ = brain.load_model(model_path);
            self.router.register_local(Box::new(brain));
        }
        self
    }

    pub async fn execute(&self, ctx: ReasonCtx) -> Result<ReasonResult, BrainError> {
        let rr = self.router.route_and_execute(ctx).await?;
        Ok(rr.result)
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use hace_brain_base::ReasonCtx;

    fn ctx(action: &str, payload: &str) -> ReasonCtx {
        ReasonCtx {
            intent_id: "i1".into(), action: action.into(),
            payload: serde_json::json!({ "q": payload }),
            memory: vec![], domain: None, soul_id: None, brain_profile: None,
        }
    }

    #[tokio::test]
    async fn algo_only_mode_routes_correctly() {
        let config = ZeusConfig { mode: BrainMode::AlgoOnly, ..Default::default() };
        let mut rt = ZeusRuntime::new(config);
        rt.router.with_algo_rule("ping", serde_json::json!({"pong": true}));
        let r = rt.execute(ctx("ping", "test")).await.unwrap();
        assert_eq!(r.output["pong"], true);
        assert_eq!(r.model_id, "algo");
    }

    #[tokio::test]
    async fn hybrid_simple_falls_to_algo() {
        let rt = ZeusRuntime::new(ZeusConfig::default());
        let r = rt.execute(ctx("compute", "x")).await.unwrap();
        assert_eq!(r.model_id, "algo");
    }

    #[tokio::test]
    async fn hybrid_medium_falls_back_when_no_local_ce() {
        let rt = ZeusRuntime::new(ZeusConfig::default());
        // No local CE registered — should fall to algo
        let long_payload = "x".repeat(300);
        let r = rt.execute(ctx("infer", &long_payload)).await.unwrap();
        assert_eq!(r.model_id, "algo"); // graceful fallback
    }

    #[test]
    fn select_ce_routing_table() {
        let mut router = ZeusRouter::new(ZeusConfig::default());
        assert_eq!(router.select_ce(Complexity::Simple),  CeId::Algo);
        assert_eq!(router.select_ce(Complexity::Medium),  CeId::Local);
        assert_eq!(router.select_ce(Complexity::Complex), CeId::Local);
    }
}
