use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub document: Document,
    #[allow(unused)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(rename = "page")]
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    #[serde(rename = "page_pdf")]
    pub file_name: String,
}
