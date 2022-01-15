//! app pages
#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize)
}

pub mod project;
pub mod interface;
