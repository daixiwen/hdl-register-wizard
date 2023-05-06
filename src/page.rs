//! app pages
#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize),
    Register(usize, usize),
}

pub mod interface;
pub mod project;
pub mod register;
