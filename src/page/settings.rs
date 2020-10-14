#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use super::super::Model;
use super::super::Msg;

pub fn view(_model: &Model) -> Node<Msg> {
  div!["Settings window"]
}
