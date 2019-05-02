//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
  /// The primary colors according to the RYB color model.
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }

  /// The secondary colors according to the RYB color model.
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;

  /// Combines two primary colors in equal amounts to crate
  /// a secondary color.
  pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
    SecondaryColor::Orange
  }
}
