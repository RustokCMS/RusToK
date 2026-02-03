use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Hero,
    Text,
    Image,
    Gallery,
    Cta,
    Features,
    Testimonials,
    Pricing,
    Faq,
    Contact,
    ProductGrid,
    Newsletter,
    Video,
    Html,
    Spacer,
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBlockInput {
    pub block_type: BlockType,
    pub position: i32,
    pub data: serde_json::Value,
    pub translations: Option<Vec<BlockTranslationInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockTranslationInput {
    pub locale: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBlockInput {
    pub position: Option<i32>,
    pub data: Option<serde_json::Value>,
    pub translations: Option<Vec<BlockTranslationInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockResponse {
    pub id: Uuid,
    pub block_type: BlockType,
    pub position: i32,
    pub data: serde_json::Value,
    pub translations: Option<Vec<BlockTranslationInput>>,
}
