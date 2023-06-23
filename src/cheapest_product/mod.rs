use crate::data;
use crate::scraping;

#[derive(Debug)]
struct CheapestProductError;

impl From<CheapestProductError> for anyhow::Error {
    fn from(value: CheapestProductError) -> Self {
        anyhow::Error::msg("Failed to get cheapest product")
    }
}

pub fn get_cheapest_product(product_name: &str) -> Result<data::Product, CheapestProductError> {
    let tokopedia_products = scraping::tokopedia::get_products(product_name)
        .map_err(|_| CheapestProductError)?;

    let mut products = Vec::new();
    products.extend(tokopedia_products.into_iter());

    products
        .into_iter()
        .min_by(|x, y| x.price_in_idr.partial_cmp(&y.price_in_idr).unwrap())
        .ok_or(CheapestProductError)
}
