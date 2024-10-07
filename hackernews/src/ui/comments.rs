#![allow(non_snake_case)]

use crate::Comment;
use dioxus::prelude::*;

use super::CommentsState;

/// Comments
#[component]
pub fn Comments() -> Element {
    // 获取CommentsState
    let comments_state = use_context::<Signal<CommentsState>>();

    // 根据不同准改天 返回不同结果
    match comments_state() {
        CommentsState::Unset => rsx! {
          div {}
        },
        CommentsState::Loading => rsx! {
          div { class: "mt-6",
            p { "Loading comments..." }
          }
        },
        CommentsState::Loaded(data) => rsx! {
          ul {
            // 这里产生的组件，会直接添加到UL下
            for comment in data.comments {
              StoryComment { comment }
            }
          }
        },
    }
}

/// Comment乡宁
#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
      li {
        article { class: "p-4 leading-7 tracking-wider text-gray-500 border-b border-gray-200",
          span { "{comment.by} {comment.time} | next [-]" }
          div { dangerous_inner_html: comment.text }
        }
      }
    }
}
