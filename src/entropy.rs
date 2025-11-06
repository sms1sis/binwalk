use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlockEntropy {
    pub end: usize,
    pub start: usize,
    pub entropy: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileEntropy {
    pub file: String,
    pub blocks: Vec<BlockEntropy>,
}
