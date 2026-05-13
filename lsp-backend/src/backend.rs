pub mod handlers;
pub mod documents;

use tower_lsp::Client;
use crate::backend::documents::Documents;

pub struct Backend {
    pub client: Client,
    pub documents: Documents,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Documents::new(),
        }
    }
}
