use xcap::Monitor;

use crate::cv::MatFromImage;

pub fn screenshot(monitor: &Monitor) -> Result<MatFromImage, xcap::XCapError> {
    let image = monitor.capture_image()?;
    Ok(MatFromImage::from_image(image))
}
