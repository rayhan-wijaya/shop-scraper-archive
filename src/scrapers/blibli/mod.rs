use crate::scrapers::Product;
use crate::scrapers::ScrapeError;
use crate::scrapers::parse_document_from_url;
use crate::scrapers::parse_selector;
use crate::scrapers::get_first_text_from_parent_element_selector;

pub fn get_blibli_products(search_query: &str) -> Result<Vec<Product>, ScrapeError> {
    let mut products: Vec<Product> = Vec::new();

    let unformatted_url = "
        https://www.blibli.com/cari/%s
    ";

    let url = unformatted_url
        .trim()
        .replace("%s", search_query)
        .replace(" ", "");

    let document = parse_document_from_url(&url)?;

    let product_selector = parse_selector(r#"div[class="product__item-container"]"#)?;
    let product_name_selector = parse_selector(r#"div[class="product__title"]"#)?;
    let product_price_selector = parse_selector(r#"strong[class="product__body__price__display"]"#)?;
    let product_stars_selector = parse_selector(r#"span[class="product__body__rating__stars__rating"]"#)?;

    for product_element in document.select(&product_selector) {
        let product_name = get_first_text_from_parent_element_selector(
            &product_name_selector,
            product_element
        )?;

        let product_price_in_idr = get_first_text_from_parent_element_selector(
            &product_price_selector,
            product_element
        )?
            .replace("Rp", "")
            .replace(".", "")
            .parse::<i32>()
            .map_err(|_| ScrapeError::ParseElementNodeError)?;

        let product_stars_text = get_first_text_from_parent_element_selector(
            &product_stars_selector,
            product_element
        );

        let mut product_stars = None;

        // TODO: Refrain from using `match { ... Err(_) => {} }`
        match product_stars_text {
            Ok(product_stars_text) => {
                product_stars = product_stars_text
                    .parse::<f32>()
                    .ok();
            },
            Err(_) => {},
        }

        products.push(Product {
            id: None,
            name: String::from(product_name),
            price_in_idr: product_price_in_idr,
            stars: product_stars,
        });
    }

    Ok(products)
}
