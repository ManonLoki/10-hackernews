#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use hackernews::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
