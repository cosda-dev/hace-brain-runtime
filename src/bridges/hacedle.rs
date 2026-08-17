// HacedleBridge - Bridge to Hacedle CE for edge LLM inference

use crate::{RuntimeSio, RuntimeOutcome, RuntimeBridge, Telemetry};

#[cfg(feature = "hacedle")]
use hacedle::x::provider::candle::{InferenceEngine, TokenizerEngine};

/// HacedleBridge - Bridge to Hacedle CE for edge LLM inference
pub struct HacedleBridge {
    model_path: Option<String>,
    #[cfg(feature = "hacedle")]
    engine: Option<InferenceEngine>,
}

impl HacedleBridge {
    pub fn new() -> Self {
        #[cfg(feature = "hacedle")]
        Self { 
            model_path: None,
            engine: None,
        }
        #[cfg(not(feature = "hacedle"))]
        Self { 
            model_path: None,
        }
    }

    pub fn with_model(path: &str) -> Self {
        Self::new()
    }

    /// Load model into engine
    pub fn load_model(&mut self, path: &str) -> Result<(), &'static str> {
        #[cfg(feature = "hacedle")]
        {
            let mut engine = InferenceEngine::default();
            engine.load_model(path)?;
            self.engine = Some(engine);
            self.model_path = Some(path.to_string());
            Ok(())
        }
        #[cfg(not(feature = "hacedle"))]
        {
            let _ = path;
            Err("hacedle_feature_not_enabled")
        }
    }

    /// Execute mock inference - validates full chain
    #[cfg(not(feature = "hacedle"))]
    fn execute_mock(&self, prompt: &str) -> Result<Vec<u32>, &'static str> {
        // Echo tokens - validates full chain works
        Ok(prompt.bytes().map(|b| b as u32).collect())
    }

    /// Execute real inference using Hacedle engine
    #[cfg(feature = "hacedle")]
    fn execute_inference(&self, prompt: &str, max_tokens: u32) -> Result<Vec<u32>, &'static str> {
        match &self.engine {
            Some(engine) => Ok(engine.infer(prompt, max_tokens)),
            None => Err("engine_not_loaded"),
        }
    }
}

impl Default for HacedleBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBridge for HacedleBridge {
    fn invoke(&self, sio: RuntimeSio) -> Result<RuntimeOutcome, &'static str> {
        let prompt = sio.prompt.unwrap_or_default();
        let max_tokens = sio.max_tokens.unwrap_or(64);

        #[cfg(feature = "hacedle")]
        let tokens = self.execute_inference(&prompt, max_tokens)?;

        #[cfg(not(feature = "hacedle"))]
        let tokens = self.execute_mock(&prompt)?;

        let result_tokens: Vec<u32> = tokens.into_iter().take(max_tokens as usize).collect();

        Ok(RuntimeOutcome {
            status: String::from("success"),
            result: result_tokens,
            telemetry: Telemetry {
                latency_ms: 0,
                tokens_used: result_tokens.len() as u32,
            },
        })
    }
}