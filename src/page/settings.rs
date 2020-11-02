//! settings page (TBD)

#![allow(clippy::wildcard_imports)]

use super::super::Model;
use super::super::Msg;
use seed::{prelude::*, *};

pub fn view(_model: &Model) -> Node<Msg> {
    div!["Settings window"]
}
