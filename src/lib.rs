use std::fmt::{Debug, Display};

/// ```
/// # use spellbook::Burger;
/// let _ = Burger;
/// assert_eq!(1, 1);
/// ```
pub struct Burger;

#[cfg(feature = "fries")]
pub struct Fries;

#[cfg(feature = "gyro")]
pub struct Gyro;

#[derive(Debug)]
pub struct Soup;

impl Default for Soup {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Soup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Soup {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn from(_: &str) -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct Lunch(pub Soup);

impl Display for Lunch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
