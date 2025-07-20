use crate::infrastructure::exchange::bybit::api::models::Category;
use std::fmt::Display;

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Category::Spot => "spot".to_string(),
            Category::Linear => "linear".to_string(),
            Category::Inverse => "inverse".to_string(),
            Category::Option => "option".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        match self {
            Category::Spot => "spot",
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Option => "option",
        }
    }
}
