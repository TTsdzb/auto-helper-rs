use log::{debug, trace};
use thiserror::Error;

use crate::structs::Point;
use image::{DynamicImage, ImageError, ImageReader};
use std::io::Cursor;
use std::{io, process::Command};

#[derive(Error, Debug)]
pub enum AdbScreenshotError {
    #[error("Adb screenshot command invoke failed: {0}")]
    CommandError(#[from] io::Error),
    #[error("Adb screenshot command failed with return code {code:?}")]
    ExecutionError {
        code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    #[error("Failed to decode screenshot image: {0}")]
    ImageDecodeError(#[from] ImageError),
}

pub fn screenshot() -> Result<DynamicImage, AdbScreenshotError> {
    debug!("Running adb command for screenshot...");
    let output = Command::new("adb")
        .arg("shell")
        .arg("screencap")
        .arg("-p")
        .output()?;

    if !output.status.success() {
        let stdout =
            String::from_utf8(output.stdout).unwrap_or("<Failed to decode string>".to_string());
        let stderr =
            String::from_utf8(output.stderr).unwrap_or("<Failed to decode string>".to_string());
        trace!("Stdout: {}", &stdout);
        trace!("Stderr: {}", &stderr);
        return Err(AdbScreenshotError::ExecutionError {
            code: output.status.code(),
            stdout,
            stderr,
        });
    }

    Ok(ImageReader::new(Cursor::new(output.stdout))
        .with_guessed_format()?
        .decode()?)
}

#[derive(Error, Debug)]
pub enum AdbTapError {
    #[error("Adb tap command invoke failed: {0}")]
    CommandError(#[from] io::Error),
    #[error("Adb tap command failed with return code {code:?}")]
    ExecutionError {
        code: Option<i32>,
        stdout: String,
        stderr: String,
    },
}

pub fn tap(point: &Point) -> Result<(), AdbTapError> {
    debug!("Clicking on adb device at ({}, {})", &point.x, &point.y);
    let output = Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("tap")
        .arg(format!("{}", point.x))
        .arg(format!("{}", point.y))
        .output()?;

    if !output.status.success() {
        let stdout =
            String::from_utf8(output.stdout).unwrap_or("<Failed to decode string>".to_string());
        let stderr =
            String::from_utf8(output.stderr).unwrap_or("<Failed to decode string>".to_string());
        trace!("Stdout: {}", &stdout);
        trace!("Stderr: {}", &stderr);
        return Err(AdbTapError::ExecutionError {
            code: output.status.code(),
            stdout,
            stderr,
        });
    }

    Ok(())
}
