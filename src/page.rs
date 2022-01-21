//! app pages
#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize),
    Register(usize, usize)
}

pub mod project;
pub mod interface;
pub mod register;

