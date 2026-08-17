#[derive(Debug, Clone)]
pub struct PromptContext {
    pub system: String,
    pub user: String,
    pub history: Vec<String>,
}

impl PromptContext {
    pub fn new(system: String, user: String) -> Self {
        Self {
            system,
            user,
            history: Vec::new(),
        }
    }

    pub fn with_history(mut self, history: Vec<String>) -> Self {
        self.history = history;
        self
    }

    pub fn add_history(&mut self, msg: String) {
        self.history.push(msg);
    }
}