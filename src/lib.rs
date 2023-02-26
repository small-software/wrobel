//! # Wrobel
//!
//! `wrobel` is a collection of components for yew

/// Adds one to the number given.
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = wrobel::to_check(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn to_check(x: i32) -> i32 {
    x + 1
}

pub mod wrobel;