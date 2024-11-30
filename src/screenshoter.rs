use core::time;
use std::thread;

use log::debug;
use opencv::core::{MatTraitConst, ToInputArray};
use thiserror::Error;
use xcap::Monitor;

use crate::{
    adb_commands,
    cv::{self, MatchResult},
    xcap_screenshot,
};

#[derive(Error, Debug)]
pub enum ScreenshotError {
    #[error("Failed to capture screenshot from adb: {0:?}")]
    AdbScreenshotError(#[from] adb_commands::AdbScreenshotError),
    #[error("Failed to capture screenshot from a local monitor: {0:?}")]
    XcapScreenshotError(#[from] xcap::XCapError),
}

#[derive(Error, Debug)]
pub enum FindTemplateError {
    #[error("Screenshot failed when finding template on the screen: {0:?}")]
    ScreenshotError(#[from] ScreenshotError),
    #[error("Error occurred during matching when finding template on the screen")]
    MatchTemplateError(#[from] opencv::Error),
}

pub trait Screenshoter {
    fn screenshot(&self) -> Result<impl MatTraitConst + ToInputArray, ScreenshotError>;

    fn find_template(
        &self,
        template: &(impl MatTraitConst + ToInputArray),
    ) -> Result<MatchResult, FindTemplateError> {
        let scr = self.screenshot()?;
        Ok(cv::cv_match_template_center(&scr, template)?)
    }

    fn find_template_existence(
        &self,
        template: &(impl MatTraitConst + ToInputArray),
        threshold: f32,
    ) -> Result<Option<MatchResult>, FindTemplateError> {
        let find_res = self.find_template(template)?;
        if find_res.correlation > threshold {
            debug!(
                "Found correlation is {} larger than {}",
                find_res.correlation, threshold
            );
            Ok(Some(find_res))
        } else {
            debug!(
                "Found correlation is {} smaller than {}, ignoring",
                find_res.correlation, threshold
            );
            Ok(None)
        }
    }

    fn wait_template_existence(
        &self,
        template: &(impl MatTraitConst + ToInputArray),
        threshold: f32,
        interval: time::Duration,
    ) -> Result<MatchResult, FindTemplateError> {
        debug!("Waiting for a template...");
        let mut res = self.find_template_existence(template, threshold)?;
        while res.is_none() {
            thread::sleep(interval);
            res = self.find_template_existence(template, threshold)?;
        }
        debug!("Template found, finish waiting");
        Ok(res.unwrap()) // Should be `Some` here
    }
}

#[derive(Default)]
pub struct AdbScreenshoter;

impl Screenshoter for AdbScreenshoter {
    fn screenshot(&self) -> Result<impl MatTraitConst + ToInputArray, ScreenshotError> {
        Ok(adb_commands::screenshot()?)
    }
}

pub struct XcapScreenshoter {
    monitor: Monitor,
}

impl XcapScreenshoter {
    pub fn new(monitor: Monitor) -> Self {
        Self { monitor }
    }
}

impl Screenshoter for XcapScreenshoter {
    fn screenshot(&self) -> Result<impl MatTraitConst + ToInputArray, ScreenshotError> {
        Ok(xcap_screenshot::screenshot(&self.monitor)?)
    }
}
