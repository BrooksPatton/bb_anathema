use anathema::{component::Component, state::State};

use crate::messages::Message;

pub struct ArticleTitles;

impl Component for ArticleTitles {
    type State = ArticleTitlesState;

    type Message = Message;
}

impl ArticleTitles {
    pub fn new() -> Self {
        Self
    }
}

#[derive(State)]
pub struct ArticleTitlesState {}

impl ArticleTitlesState {
    pub fn new() -> Self {
        Self {}
    }
}
