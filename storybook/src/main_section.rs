use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::{articles::get_article_content, messages::Message};

pub struct Main;

impl Component for Main {
    type State = MainState;

    type Message = Message;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match message {
            Message::ArticleTitleClicked(title) => {
                let content = get_article_content(&title).unwrap().unwrap();

                state.route.set("article".to_owned());
                state.article_title.set(title);
                state.article_content.set(content);
            }
        }
    }
}

#[derive(State)]
pub struct MainState {
    pub route: Value<String>,
    pub article_title: Value<String>,
    pub article_content: Value<String>,
}

impl MainState {
    pub fn new() -> Self {
        let route = "".to_owned().into();
        let article_title = "".to_owned().into();
        let article_content = "".to_owned().into();

        Self {
            route,
            article_title,
            article_content,
        }
    }
}
