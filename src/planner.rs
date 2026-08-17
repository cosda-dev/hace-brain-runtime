// TeleportPlan - Canonical output structure for Zeus
// CRD Directive D7: Zeus returns TeleportPlan, not raw text

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct TeleportPlan {
    pub objective: String,
    pub steps: Vec<String>,
    pub confidence: f32,
    pub model_used: String,
    pub tokens_generated: u32,
}

impl TeleportPlan {
    pub fn new(objective: &str) -> Self {
        Self {
            objective: objective.to_string(),
            steps: Vec::new(),
            confidence: 0.0,
            model_used: String::new(),
            tokens_generated: 0,
        }
    }

    pub fn with_steps(mut self, steps: Vec<&str>) -> Self {
        self.steps = steps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_confidence(mut self, conf: f32) -> Self {
        self.confidence = conf;
        self
    }

    pub fn build_loader_plan() -> Self {
        Self::new("build_loader")
            .with_steps(vec![
                "parse_gguf_header",
                "parse_tensor_index",
                "mmap_tensor_data",
                "verify_offsets",
            ])
            .with_confidence(0.91)
    }
}

impl Default for TeleportPlan {
    fn default() -> Self {
        Self::new("default")
    }
}