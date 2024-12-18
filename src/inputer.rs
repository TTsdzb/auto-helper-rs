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
    #[cfg(target_os = "windows")]
    scale_factor: f32,
}

impl EnigoInputer {
    pub fn new(
        settings: &enigo::Settings,
        #[cfg(target_os = "windows")] scale_factor: f32,
    ) -> Result<Self, enigo::NewConError> {
        Ok(Self {
            enigo: Enigo::new(settings)?,
            #[cfg(target_os = "windows")]
            scale_factor,
        })
    }

    pub fn default() -> Result<Self, enigo::NewConError> {
        Ok(Self {
            enigo: Enigo::new(&enigo::Settings::default())?,
            #[cfg(target_os = "windows")]
            scale_factor: 1.0,
        })
    }

    #[cfg(target_os = "windows")]
    pub fn default_with_factor(scale_factor: f32) -> Result<Self, enigo::NewConError> {
        Ok(Self {
            enigo: Enigo::new(&enigo::Settings::default())?,
            #[cfg(target_os = "windows")]
            scale_factor,
        })
    }
}

impl Inputer for EnigoInputer {
    fn click(&mut self, pos: &Point) -> Result<(), InputError> {
        debug!("Clicking at ({}, {})", pos.x, pos.y);

        #[cfg(target_os = "windows")]
        let (x, y) = (
            (pos.x as f32 / self.scale_factor) as i32,
            (pos.y as f32 / self.scale_factor) as i32,
        );
        #[cfg(not(target_os = "windows"))]
        let (x, y) = (pos.x as i32, pos.y as i32);

        self.enigo.move_mouse(x, y, enigo::Coordinate::Abs)?;
        self.enigo
            .button(enigo::Button::Left, enigo::Direction::Click)?;

        Ok(())
    }
}
