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

const MATCH_SIZE_THRESHOLD: u32 = 500;

pub fn cv_match_template_center(source: &DynamicImage, template: &DynamicImage) -> MatchResult {
    let template_width = template.width();
    let template_height = template.height();

    let mut source_grayscale = source.to_luma8();
    let mut template_grayscale = template.to_luma8();
    let mut scale = 1;
    while source_grayscale.width() > MATCH_SIZE_THRESHOLD
        || source_grayscale.height() > MATCH_SIZE_THRESHOLD
    {
        source_grayscale = imageops::resize(
            &source_grayscale,
            source_grayscale.width() / 2,
            source_grayscale.height() / 2,
            FilterType::Nearest,
        );
        template_grayscale = imageops::resize(
            &template_grayscale,
            template_grayscale.width() / 2,
            template_grayscale.height() / 2,
            FilterType::Nearest,
        );
        scale *= 2;
    }

    let match_result = template_matching::match_template_parallel(
        &source_grayscale,
        &template_grayscale,
        MatchTemplateMethod::CrossCorrelationNormalized,
    );
    let extremes = template_matching::find_extremes(&match_result);

    let x = extremes.max_value_location.0 * scale + template_width / 2;
    let y = extremes.max_value_location.1 * scale + template_height / 2;

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
