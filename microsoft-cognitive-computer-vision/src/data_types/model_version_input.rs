use core::fmt;

#[derive(Debug, Clone)]
pub enum ModelVersionInput {
    Latest,
    Specific(String),
}

impl fmt::Display for ModelVersionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Latest => write!(f, "latest"),
            Self::Specific(str) => write!(f, "{}", str),
        }
    }
}

impl Default for ModelVersionInput {
    fn default() -> Self {
        Self::Latest
    }
}
