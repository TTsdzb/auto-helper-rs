use enigo::{Enigo, Mouse};
use log::debug;
use thiserror::Error;

use crate::{adb_commands, structs::Point};

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Failed to click on adb device: {0:?}")]
    AdbInputError(#[from] adb_commands::AdbTapError),
    #[error("Failed to click on local device: {0:?}")]
    EnigoInputError(#[from] enigo::InputError),
}

pub trait Inputer {
    fn click(&mut self, pos: &Point) -> Result<(), InputError>;
}

#[derive(Default)]
pub struct AdbInputer;

impl Inputer for AdbInputer {
    fn click(&mut self, pos: &Point) -> Result<(), InputError> {
        Ok(adb_commands::tap(pos)?)
    }
}

pub struct EnigoInputer {
    enigo: Enigo,
}

impl EnigoInputer {
    pub fn new(settings: &enigo::Settings) -> Result<Self, enigo::NewConError> {
        Ok(Self {
            enigo: Enigo::new(settings)?,
        })
    }
}

impl Inputer for EnigoInputer {
    fn click(&mut self, pos: &Point) -> Result<(), InputError> {
        debug!("Clicking at ({}, {})", pos.x, pos.y);
        self.enigo
            .move_mouse(pos.x as i32, pos.y as i32, enigo::Coordinate::Abs)?;
        self.enigo
            .button(enigo::Button::Left, enigo::Direction::Click)?;

        Ok(())
    }
}
