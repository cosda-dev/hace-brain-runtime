use crate::{RuntimeSio, RuntimeOutcome, RuntimeExecutor};

pub struct BrainExecutor;

impl BrainExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str> {
        Ok(RuntimeOutcome {
            status: alloc::string::String::from("executed"),
            result: alloc::vec::Vec::new(),
            telemetry: Default::default(),
        })
    }
}

impl RuntimeExecutor for BrainExecutor {
    fn execute(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str> {
        self.execute(sio)
    }
}

impl Default for BrainExecutor {
    fn default() -> Self {
        Self::new()
    }
}