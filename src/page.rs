//! app pages
#[derive(PartialEq)]
pub enum PageType {
    Project,
    Interface(usize)
}

pub mod project;
pub mod interface;
