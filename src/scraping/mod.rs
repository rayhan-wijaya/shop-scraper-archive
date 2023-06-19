pub mod tokopedia;
pub mod blibli;
pub mod bukalapak;

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

pub fn parse_document_from_url(url: &str) -> Result<scraper::Html, ScrapeError> {
    let response = reqwest::blocking::get(url)
        .map_err(|_| ScrapeError::RequestError)?;

    let response_string = response
        .text()
        .map_err(|_| ScrapeError::ParseRequestError)?;

    Ok(scraper::Html::parse_document(&response_string))
}

pub fn parse_selector(selector_string: &str) -> Result<scraper::Selector, ScrapeError> {
    let selector = scraper::Selector::parse(selector_string)
        .map_err(|_| ScrapeError::ParseSelectorError)?;

    Ok(selector)
}

pub fn get_first_text_from_selector<'a>(
    selector: &scraper::Selector,
    parent_element: scraper::ElementRef<'a>
) -> Option<&'a str> {
    parent_element
        .select(selector)
        .next()
        .and_then(|element| element.text().next())
}
