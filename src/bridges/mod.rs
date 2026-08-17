mod hacedle;
mod hacetral;

pub use hacedle::HacedleBridge;
pub use hacetral::HacetralBridge;

use crate::RuntimeSio;
use crate::RuntimeOutcome;

pub trait RuntimeBridge {
    fn invoke(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str>;
}