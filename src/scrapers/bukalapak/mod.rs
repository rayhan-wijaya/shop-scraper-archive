use crate::scrapers::Product;
use crate::scrapers::ScrapeError;
use crate::scrapers::parse_document_from_url;
use crate::scrapers::parse_selector;
use crate::scrapers::get_first_text_from_parent_element_selector;

pub fn get_bukalapak_products(search_query: &str) -> Result<Vec<Product>, ScrapeError> {
    let mut products: Vec<Product> = Vec::new();
    
    let unformatted_url = "
        https://www.bukalapak.com/products
            ?search%5Bkeywords%5D=%s
            &search%5Btop_seller%5D=1
            &search%5Bbrand%5D=1
    ";

    let url = unformatted_url
        .trim()
        .replace(" ", "")
        .replace("%s", search_query);

    let document = parse_document_from_url(&url)?;

    let product_selector = parse_selector(r#"div[class="bl-product-card-new__wrapper"]"#)?;
    let product_name_selector = parse_selector(r#"p[class="bl-text bl-text--body-14 bl-text--secondary bl-text--ellipsis__2"]"#)?;
    let product_price_selector = parse_selector(r#"p[class="bl-text bl-text--semi-bold bl-text--ellipsis__1 bl-product-card-new__price"]"#)?;
    let product_stars_selector = parse_selector(r#"p[class="bl-text bl-text--caption-12 bl-text--bold"]"#)?;
    let bukalapak_link_selector = parse_selector(r#"a[class="bl-link"]"#)?;

    for product_element in document.select(&product_selector) {
        let product_name_wrapper_element = product_element
            .select(&product_name_selector)
            .next()
            .ok_or(ScrapeError::RetrieveElementNodeError)?;

        let product_name_element = product_name_wrapper_element
            .select(&bukalapak_link_selector)
            .next()
            .ok_or(ScrapeError::RetrieveElementNodeError)?;

        let product_name = product_name_element
            .text()
            .next()
            .ok_or(ScrapeError::RetrieveElementNodeError)?;

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