use crate::data;
use crate::scraping;

pub fn get_cheapest_product(product_name: &str) -> Option<data::Product> {
    let tokopedia_products = scraping::tokopedia::get_products(product_name).ok()?;

    let mut products = Vec::new();
    products.extend(tokopedia_products.into_iter());

    products
        .into_iter()
        .min_by(|x, y| x.price_in_idr.cmp(&y.price_in_idr))
}
