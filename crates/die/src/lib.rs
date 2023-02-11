use nanorand::{Rng, WyRand};

/// Roll a die.
///
/// # Examples
///
/// ```
/// let roll = die::roll(6);
/// assert!(roll > 0);
/// assert!(roll <= 6);
/// ```
pub fn roll(sides: u8) -> u8 {
    let mut rng = WyRand::new();
    rng.generate_range(1..=sides)
}
