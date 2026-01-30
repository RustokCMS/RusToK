use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::PriceResponse;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVariantInput {
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub prices: Vec<PriceInput>,
    #[serde(default)]
    pub inventory_quantity: i32,
    #[serde(default = "default_inventory_policy")]
    pub inventory_policy: String,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
}

fn default_inventory_policy() -> String {
    "deny".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PriceInput {
    pub currency_code: String,
    pub amount: Decimal,
    pub compare_at_amount: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateVariantInput {
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub prices: Option<Vec<PriceInput>>,
    pub inventory_quantity: Option<i32>,
    pub inventory_policy: Option<String>,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VariantResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub title: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub prices: Vec<PriceResponse>,
    pub inventory_quantity: i32,
    pub inventory_policy: String,
    pub in_stock: bool,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdjustInventoryInput {
    pub variant_id: Uuid,
    pub adjustment: i32,
    pub reason: Option<String>,
}
