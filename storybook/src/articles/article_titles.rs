use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::messages::Message;

pub struct ArticleTitles;

impl Component for ArticleTitles {
    type State = ArticleTitlesState;

    type Message = Message;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match message {
            Message::ArticleTitleClicked(title) => {
                state.selected_article.set(title);
            }
            Message::ArticleTitlesLoaded(items) => todo!(),
        }
    }
}

impl ArticleTitles {
    pub fn new() -> Self {
        Self
    }
}

#[derive(State)]
pub struct ArticleTitlesState {
    pub selected_article: Value<String>,
}

impl ArticleTitlesState {
    pub fn new() -> Self {
        let selected_article = "".to_owned().into();

        Self { selected_article }
    }
}
