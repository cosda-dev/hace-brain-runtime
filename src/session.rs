use alloc::string::String;

pub struct SessionManager {
    sessions: alloc::collections::BTreeMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: alloc::collections::BTreeMap::new(),
        }
    }
}

pub struct Session {
    pub id: String,
    pub state: SessionState,
}

pub enum SessionState {
    Pending,
    Running,
    Completed,
}