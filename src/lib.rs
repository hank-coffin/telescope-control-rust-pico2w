#![cfg_attr(not(test), no_std)]

pub mod astronomy;
pub mod motor;
//pub mod ui;

// Re-export main modules
pub use astronomy::*;
pub use motor::*;
//pub use ui::*;

// Common types and constants
pub type Degrees = f32;
pub type Radians = f32;
pub type Steps = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub azimuth: Degrees,
    pub altitude: Degrees,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorPosition {
    pub azimuth_steps: Steps,
    pub altitude_steps: Steps,
}