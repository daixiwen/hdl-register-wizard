use std::fmt;
use crate::page::PageType;
use std::error::Error;

pub struct GenError {
    pub page: PageType,
    pub message: String,
}

// Different error messages according to AppError.code
impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generate error: {}", self.message)
    }
}

// A unique format for dubugging output
impl fmt::Debug for GenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Generate error {{ page: {}, message: {} }}",
            match self.page {
                PageType::Project => "project".to_owned(),
                PageType::Interface(int) => format!("interface({})", int),
                PageType::Register(int,reg, field) => format!("interface({}), register({}), field({:?})", int, reg, field)
            },
            self.message
        )
    }
}

impl Error for GenError {

}

impl GenError {
    pub fn new(page: &PageType, message: &str) -> Self {
        Self {
            page: page.clone(),
            message: message.to_owned()
        }
    }
}

