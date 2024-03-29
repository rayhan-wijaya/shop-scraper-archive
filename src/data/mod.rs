#[derive(Debug, serde::Serialize)]
pub struct Product {
    pub id: Option<String>,
    pub name: String,
    pub stars: Option<f32>,
    pub price_in_idr: i32,
}

