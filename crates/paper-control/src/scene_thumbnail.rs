use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneThumbnailRequest {
    pub source: String,
    pub destination: String,
    #[serde(default)]
    pub reference: bool,
    /// Exact capture size in pixels (e.g. a monitor's resolution). Overrides the
    /// default thumbnail size; the result is written as a lossless PNG.
    #[serde(default)]
    pub size: Option<(u32, u32)>,
    /// Fill mode name (`fill`, `fit`, `stretch`, ...) used when composing a scene.
    #[serde(default)]
    pub fill: Option<String>,
    #[serde(default)]
    pub properties: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneThumbnailResponse {
    pub source: String,
    pub error: Option<String>,
}
