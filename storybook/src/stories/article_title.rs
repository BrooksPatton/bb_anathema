use crate::messages::Message;
use anathema::{component::Component, state::State};

pub struct ArticleTitle;

impl Component for ArticleTitle {
    type State = ArticleTitleState;

    type Message = Message;

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        if mouse.lsb_down() {
            children
                .elements()
                .at_pos(mouse.pos())
                .first(|_elements, attributes| {
                    let title = attributes
                        .get("title")
                        .unwrap()
                        .as_str()
                        .map(String::from)
                        .unwrap_or_default();
                    let message = Message::ArticleTitleClicked(title);

                    context.components.by_name("main").send(message.clone());
                    context.components.by_name("article_titles").send(message);
                });
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match message {
            Message::ArticleTitleClicked(_title) => (),
            Message::ArticleTitlesLoaded(_items) => todo!(),
        }
    }
}

#[derive(State)]
pub struct ArticleTitleState {}

impl ArticleTitleState {
    pub fn new() -> Self {
        Self {}
    }
}
