use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{CreateVariantInput, VariantResponse};
use crate::entities::product::ProductStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct CreateProductInput {
    pub translations: Vec<ProductTranslationInput>,
    #[serde(default)]
    pub options: Vec<ProductOptionInput>,
    pub variants: Vec<CreateVariantInput>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub publish: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductTranslationInput {
    pub locale: String,
    pub title: String,
    pub handle: Option<String>,
    pub description: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOptionInput {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct UpdateProductInput {
    pub translations: Option<Vec<ProductTranslationInput>>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub status: Option<ProductStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub status: ProductStatus,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub translations: Vec<ProductTranslationResponse>,
    pub options: Vec<ProductOptionResponse>,
    pub variants: Vec<VariantResponse>,
    pub images: Vec<ProductImageResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductTranslationResponse {
    pub locale: String,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOptionResponse {
    pub id: Uuid,
    pub name: String,
    pub values: Vec<String>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductImageResponse {
    pub id: Uuid,
    pub media_id: Uuid,
    pub url: String,
    pub alt_text: Option<String>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PriceResponse {
    pub currency_code: String,
    pub amount: Decimal,
    pub compare_at_amount: Option<Decimal>,
    pub on_sale: bool,
}
