use std::ops::{Deref, DerefMut};

use log::{debug, trace};
use opencv::{core::ToInputArray, imgproc, prelude::*};
use thiserror::Error;
use xcap::image::RgbImage;

use crate::structs::Point;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MatchResult {
    pub pos: Point,
    pub correlation: f32,
}

pub struct MatFromImage {
    _buffer: Vec<u8>,
    pub mat: Mat,
}

impl MatFromImage {
    pub fn from_rgb_image(mut image: RgbImage) -> Self {
        // Swap RGB to BGR for OpenCV
        // This is a bit hacky since `image` crate has no BGR support
        for pixel in image.pixels_mut() {
            let orig_red = pixel.0[0];
            pixel.0[0] = pixel.0[2];
            pixel.0[2] = orig_red;
        }
        let (width, height) = image.dimensions();
        let raw_pixels = image.into_raw();
        Self {
            mat: unsafe {
                Mat::new_rows_cols_with_data_unsafe(
                    height as i32,
                    width as i32,
                    opencv::core::CV_8UC3, // Four 8 bit channel per pixel
                    raw_pixels.as_ptr() as *mut core::ffi::c_void,
                    3 * width as usize, // Bytes shift per line
                )
                .unwrap()
            },
            _buffer: raw_pixels,
        }
    }
}

impl Deref for MatFromImage {
    type Target = Mat;

    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}

impl DerefMut for MatFromImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mat
    }
}

impl AsRef<Mat> for MatFromImage {
    fn as_ref(&self) -> &Mat {
        &self.mat
    }
}

impl ToInputArray for MatFromImage {
    fn input_array(
        &self,
    ) -> opencv::Result<opencv::boxed_ref::BoxedRef<opencv::core::_InputArray>> {
        self.mat.input_array()
    }
}

impl MatTraitConst for MatFromImage {
    fn as_raw_Mat(&self) -> *const std::ffi::c_void {
        self.mat.as_raw_Mat()
    }
}

pub fn load_image_file(path: &str) -> Result<Mat, opencv::Error> {
    debug!("Loading image file from: {}", path);
    opencv::imgcodecs::imread(path, opencv::imgcodecs::IMREAD_COLOR)
}

#[derive(Error, Debug)]
pub enum CvSaveImageError {
    #[error("Could not encode the given image: {0}")]
    ImageEncodeError(#[from] opencv::Error),
    #[error("Failed to write image file")]
    ImageWriteError,
}

pub fn save_image_file(path: &str, image: &impl ToInputArray) -> Result<(), CvSaveImageError> {
    debug!("Saving image file to: {}", path);
    let success = opencv::imgcodecs::imwrite(path, image, &opencv::core::Vector::new())?;
    match success {
        true => Ok(()),
        false => Err(CvSaveImageError::ImageWriteError),
    }
}

pub fn cv_match_template_center(
    source: &(impl MatTraitConst + ToInputArray),
    template: &(impl MatTraitConst + ToInputArray),
) -> Result<MatchResult, opencv::Error> {
    let template_width = template.cols();
    let template_height = template.rows();

    let mut result = Mat::default();
    imgproc::match_template(
        source,
        template,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &Mat::default(),
    )?;
    let mut max_val = 0f64;
    let mut max_loc = opencv::core::Point::default();
    opencv::core::min_max_loc(
        &result,
        None,
        Some(&mut max_val),
        None,
        Some(&mut max_loc),
        &Mat::default(),
    )?;
    let x = max_loc.x + template_width / 2;
    let y = max_loc.y + template_height / 2;

    trace!(
        "Template matches on ({}, {}) with correlation {}",
        x,
        y,
        max_val
    );

    Ok(MatchResult {
        pos: Point::new(x as u32, y as u32),
        correlation: max_val as f32,
    })
}
