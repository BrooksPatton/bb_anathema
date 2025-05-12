use anathema::{
    component::Component,
    state::{List, State, Value},
};
use eyre::{Context, Result};

use crate::articles::get_article_titles;

pub struct App;

impl Component for App {
    type State = AppState;

    type Message = ();
}

#[derive(State)]
pub struct AppState {
    pub article_titles: Value<List<String>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let mut article_titles = Value::new(List::empty());

        for title in get_article_titles().context("getting article titles")? {
            article_titles.push(title);
        }

        Ok(Self { article_titles })
    }
}
