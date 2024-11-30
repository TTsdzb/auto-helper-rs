use log::trace;
use xcap::{image::DynamicImage, Monitor};

use crate::cv::MatFromImage;

pub fn screenshot(monitor: &Monitor) -> Result<MatFromImage, xcap::XCapError> {
    trace!("Capture screenshot on monitor {}", monitor.name());
    let image = DynamicImage::ImageRgba8(monitor.capture_image()?).into_rgb8();
    Ok(MatFromImage::from_rgb_image(image))
}
