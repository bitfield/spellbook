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
