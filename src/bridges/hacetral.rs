use crate::{RuntimeSio, RuntimeOutcome, RuntimeBridge};

pub struct HacetralBridge;

impl RuntimeBridge for HacetralBridge {
    fn invoke(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str> {
        let _ = sio;
        Ok(RuntimeOutcome {
            status: alloc::string::String::from("workflow"),
            result: alloc::vec::Vec::new(),
            telemetry: Default::default(),
        })
    }
}