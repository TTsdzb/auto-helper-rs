use image::imageops::FilterType;
use image::{imageops, DynamicImage, ImageReader, ImageResult};
use imageproc::template_matching;
use imageproc::template_matching::MatchTemplateMethod;
use log::{debug, trace};

use crate::structs::Point;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MatchResult {
    pub pos: Point,
    pub correlation: f32,
}

pub fn load_image_file(path: &str) -> ImageResult<DynamicImage> {
    debug!("Loading image file from: {}", path);
    ImageReader::open(path)?.decode()
}

pub fn save_image_file(path: &str, image: &DynamicImage) -> ImageResult<()> {
    debug!("Saving image file to: {}", path);
    image.save(path)
}

pub fn cv_match_template_center(
    source: &DynamicImage,
    template: &DynamicImage,
    match_size_threshold: u32,
) -> MatchResult {
    let template_width = template.width();
    let template_height = template.height();

    let source_width = source.width();
    let source_height = source.height();
    let mut scale = 1.0f32;
    let (source_grayscale, template_grayscale) =
        if source_width >= source_height && source_width > match_size_threshold {
            scale = source_width as f32 / match_size_threshold as f32;
            (
                imageops::resize(
                    &source.to_luma8(),
                    match_size_threshold,
                    (source.height() as f32 / scale) as u32,
                    FilterType::Nearest,
                ),
                imageops::resize(
                    &template.to_luma8(),
                    (template.width() as f32 / scale) as u32,
                    (template.height() as f32 / scale) as u32,
                    FilterType::Nearest,
                ),
            )
        } else if source_height > match_size_threshold {
            scale = source_height as f32 / match_size_threshold as f32;
            (
                imageops::resize(
                    &source.to_luma8(),
                    (source.width() as f32 / scale) as u32,
                    match_size_threshold,
                    FilterType::Nearest,
                ),
                imageops::resize(
                    &template.to_luma8(),
                    (template.width() as f32 / scale) as u32,
                    (template.height() as f32 / scale) as u32,
                    FilterType::Nearest,
                ),
            )
        } else {
            (source.to_luma8(), template.to_luma8())
        };

    let match_result = template_matching::match_template_parallel(
        &source_grayscale,
        &template_grayscale,
        MatchTemplateMethod::CrossCorrelationNormalized,
    );
    let extremes = template_matching::find_extremes(&match_result);

    let x = (extremes.max_value_location.0 as f32 * scale) as u32 + template_width / 2;
    let y = (extremes.max_value_location.1 as f32 * scale) as u32 + template_height / 2;

    trace!(
        "Template matches on ({}, {}) with correlation {}",
        x,
        y,
        extremes.max_value
    );

    MatchResult {
        pos: Point::new(x, y),
        correlation: extremes.max_value,
    }
}
