pub use xcap::{image, Monitor};

pub mod adb_commands;
pub mod cv;
pub mod structs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests;
