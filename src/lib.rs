pub use enigo;
pub use opencv::prelude as cv_prelude;
pub use opencv::Error as CvError;
pub use xcap::{image, Monitor, XCapError};

pub mod adb_commands;
pub mod cv;
pub mod inputer;
pub mod screenshoter;
pub mod structs;

#[cfg(test)]
mod tests;
