use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::Url;

pub struct Documents {
    pub map: RwLock<HashMap<Url, String>>,
}

impl Documents {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}
