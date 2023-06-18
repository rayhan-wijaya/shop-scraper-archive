pub fn parse_blibli_document(search_query: &str) -> Result<scraper::Html, super::ScrapeError> {
    let unformatted_url = "
        https://www.blibli.com/cari/%s
    ";

    let url = unformatted_url
        .trim()
        .replace("%s", search_query)
        .replace(" ", "");

    super::parse_document_from_url(&url)
}

pub fn get_blibli_products(search_query: &str) -> Result<Vec<super::Product>, super::ScrapeError> {
    let mut products: Vec<super::Product> = Vec::new();
    let document = parse_blibli_document(search_query)?;

    let product_selector = super::parse_selector(r#"div[class="product__item-container"]"#)?;
    let product_name_selector = super::parse_selector(r#"div[class="product__title"]"#)?;
    let product_price_selector = super::parse_selector(r#"strong[class="product__body__price__display"]"#)?;
    let product_stars_selector = super::parse_selector(r#"span[class="product__body__rating__stars__rating"]"#)?;

    for product_element in document.select(&product_selector) {
        let product_name = super::get_first_text_from_parent_element_selector(
            &product_name_selector,
            product_element
        )
            .ok_or(super::ScrapeError::RetrieveElementNodeError)?;

        let product_price_in_idr = super::get_first_text_from_parent_element_selector(
            &product_price_selector,
            product_element
        )
            .ok_or(super::ScrapeError::RetrieveElementNodeError)?
            .replace("Rp", "")
            .replace(".", "")
            .parse::<i32>()
            .map_err(|_| super::ScrapeError::ParseElementNodeError)?;

        let product_stars_text = super::get_first_text_from_parent_element_selector(
            &product_stars_selector,
            product_element
        );

        let product_stars = super::get_first_text_from_parent_element_selector(
            &product_stars_selector,
            product_element
        )
            .and_then(|product_stars_text| {
                product_stars_text
                    .parse::<f32>()
                    .ok()
            });

        products.push(super::Product {
            id: None,
            name: String::from(product_name),
            price_in_idr: product_price_in_idr,
            stars: product_stars,
        });
    }

    Ok(products)
}
