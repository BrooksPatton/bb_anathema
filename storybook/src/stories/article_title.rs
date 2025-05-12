use anathema::component::Component;

use crate::messages::Message;

pub struct ArticleTitle;

impl Component for ArticleTitle {
    type State = ();

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        if mouse.lsb_up() {
            children
                .elements()
                .at_pos(mouse.pos())
                .each(|_elements, attributes| {
                    let title = attributes
                        .get("title")
                        .unwrap()
                        .as_str()
                        .map(String::from)
                        .unwrap_or_default();

                    context
                        .components
                        .by_name("main")
                        .send(Message::ArticleTitleClicked(title));
                });
        }
    }
}
