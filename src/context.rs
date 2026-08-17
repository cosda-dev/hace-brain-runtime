use crate::RuntimeSio;

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build(sio: RuntimeSio) -> RuntimeContext {
        RuntimeContext {
            sio,
            initialized: true,
        }
    }
}

pub struct RuntimeContext {
    pub sio: RuntimeSio,
    pub initialized: bool,
}