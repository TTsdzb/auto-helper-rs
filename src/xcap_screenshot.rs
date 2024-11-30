use log::debug;
use xcap::Monitor;

use crate::cv::MatFromImage;

pub fn screenshot(monitor: &Monitor) -> Result<MatFromImage, xcap::XCapError> {
    debug!("Capture screenshot on monitor {}", monitor.name());
    let image = monitor.capture_image()?;
    Ok(MatFromImage::from_image(image))
}
