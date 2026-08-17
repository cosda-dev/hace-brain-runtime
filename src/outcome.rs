use alloc::string::String;
use alloc::vec::Vec;

pub struct OutcomeBuilder;

#[derive(Debug, Clone, Default)]
pub struct Telemetry {
    pub latency_ms: u64,
    pub tokens_used: u32,
}

impl OutcomeBuilder {
    pub fn build(status: &str) -> RuntimeOutcome {
        RuntimeOutcome {
            status: status.to_string(),
            result: Vec::new(),
            telemetry: Telemetry::default(),
        }
    }
}