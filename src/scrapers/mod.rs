#[derive(Debug)]
pub struct Product {
    pub id: Option<String>,
    pub name: String,
    pub stars: Option<f32>,
    pub price_in_idr: i32,
}

#[derive(Debug)]
pub enum ScrapeError {
    RequestError,
    ParseRequestError,
    ParseSelectorError,
    RetrieveElementNodeError,
    ParseElementNodeError,
}
