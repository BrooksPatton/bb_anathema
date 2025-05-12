use anathema::{
    component::Component,
    state::{List, State, Value},
};
use eyre::Result;

use crate::messages::Message;

pub struct App;

impl Component for App {
    type State = AppState;

    type Message = Message;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match message {
            Message::ArticleTitleClicked(_article_title) => (),
            Message::ArticleTitlesLoaded(article_titles) => {
                let mut wrapped_article_titles = List::empty();

                article_titles
                    .into_iter()
                    .for_each(|title| wrapped_article_titles.push(title));

                state.article_titles.set(wrapped_article_titles);
            }
        }
    }
}

#[derive(State)]
pub struct AppState {
    pub article_titles: Value<List<String>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let article_titles = Value::empty();

        Ok(Self { article_titles })
    }
}
