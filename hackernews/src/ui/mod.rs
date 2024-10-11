#![allow(non_snake_case)]
mod comments;
mod stores;

use crate::StoryData;
use comments::Comments;
use dioxus::prelude::*;
use stores::Stories;

/// Comments状态
#[derive(Debug, Clone)]
pub enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
}

pub fn App() -> Element {
    // 这里创建Provider， 所有的子组件都可以拿到这个Provider提供的State
    use_context_provider(|| Signal::new(CommentsState::Unset));
    rsx! {
      main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50 ",
        style:"overflow-y:auto;",
          Stories {}
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl",   style:"overflow-y:auto;", Comments {} }
      }
    }
}
