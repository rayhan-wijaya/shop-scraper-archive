use crate::scrapers::Product;
use crate::scrapers::ScrapeError;
use crate::scrapers::parse_document_from_url;
use crate::scrapers::parse_selector;
use crate::scrapers::get_first_text_from_parent_element;

pub fn get_products(search_query: &str) -> Result<Vec<Product>, ScrapeError> {
    let mut products: Vec<Product> = Vec::new();

    let url = format!("https://www.tokopedia.com/search?st=product&ob=3&q={}", &search_query);
    let document = parse_document_from_url(&url)?;

    let products_selector = parse_selector(r#"div[class="pcv3__container css-gfx8z3"]"#)?;
    let product_name_selector = parse_selector(r#"div[class="prd_link-product-name css-3um8ox"]"#)?;
    let product_price_selector = parse_selector(r#"div[class="prd_link-product-price css-1ksb19c"]"#)?;
    let product_stars_selector = parse_selector(r#"span[class="prd_rating-average-text css-t70v7i"]"#)?;

    for product_element in document.select(&products_selector) {
        let product_name = get_first_text_from_parent_element(
            &product_name_selector,
            product_element
        )?;

        let product_price_in_idr = get_first_text_from_parent_element(
            &product_price_selector,
            product_element
        )?
            .replace("Rp", "")
            .replace(".", "")
            .parse::<i32>()
            .map_err(|_| ScrapeError::ParseElementNodeError)?;

        let product_stars_text = get_first_text_from_parent_element(
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
