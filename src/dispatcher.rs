use crate::{RuntimeSio, RuntimeOutcome, RuntimeBridge, bridges::{HacedleBridge, HacetralBridge}};

pub struct RuntimeDispatcher {
    hacedle_bridge: HacedleBridge,
    hacetral_bridge: HacetralBridge,
}

impl RuntimeDispatcher {
    pub fn new() -> Self {
        Self {
            hacedle_bridge: HacedleBridge::new(),
            hacetral_bridge: HacetralBridge::new(),
        }
    }

    pub fn load_hacedle_model(&mut self, path: &str) -> Result<(), &'static str> {
        self.hacedle_bridge.load_model(path)
    }

    pub fn dispatch(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str> {
        // CRD Directive D2: Select CE based on runtime.provider
        match sio.runtime.provider.as_str() {
            "hacedle" => self.hacedle_bridge.invoke(sio),
            "hacetral" => self.hacetral_bridge.invoke(sio),
            _ => {
                // Default to hacedle
                self.hacedle_bridge.invoke(sio)
            }
        }
    }
}

impl Default for RuntimeDispatcher {
    fn default() -> Self {
        Self::new()
    }
}