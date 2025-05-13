#[derive(Clone)]
pub enum Message {
    ArticleTitleClicked(String),
    ArticleTitlesLoaded(Vec<String>),
}
