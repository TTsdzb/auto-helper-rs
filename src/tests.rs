use opencv::core::MatTraitConst;

use crate::{
    adb_commands, cv,
    screenshoter::{AdbScreenshoter, Screenshoter, XcapScreenshoter},
    structs::Point,
    xcap_screenshot,
};

#[test]
fn direct_adb_screenshot() {
    let image = adb_commands::screenshot().unwrap();
    cv::save_image_file("direct_adb_screenshot.png", &image).unwrap();

    println!("Execute complete, please check the image file.");
}

#[test]
fn direct_adb_tap() {
    // Settings button in control center of Mi Pad 6 Pro
    let point = Point::new(2700, 285);
    adb_commands::tap(&point).unwrap();

    println!("Execute complete, please observe device behavior.");
}

#[test]
fn direct_xcap_screenshot() {
    let monitors = xcap::Monitor::all().unwrap();
    let monitor = monitors.get(0).unwrap();

    let image = xcap_screenshot::screenshot(monitor).unwrap();
    cv::save_image_file("direct_xcap_screenshot.png", &image).unwrap();

    println!("Execute complete, please check the image file.");
}

#[test]
fn cv_match_template() {
    let source = cv::load_image_file("test_assets/image_source.png").unwrap();
    let template = cv::load_image_file("test_assets/image_template.png").unwrap();

    println!("{} {}", source.depth(), source.channels());

    let res = cv::cv_match_template_center(&source, &template).unwrap();

    println!("{:#?}", res);
}

#[test]
fn trait_xcap_screenshot() {
    let monitors = xcap::Monitor::all().unwrap();
    let monitor = monitors.into_iter().next().unwrap();
    let xcap_screenshoter = XcapScreenshoter::new(monitor);

    let image = xcap_screenshoter.screenshot().unwrap();
    cv::save_image_file("trait_xcap_screenshot.png", &image).unwrap();

    println!("Execute complete, please check the image file.");
}

#[test]
fn trait_adb_screenshot() {
    let adb_screenshoter = AdbScreenshoter::default();

    let image = adb_screenshoter.screenshot().unwrap();
    cv::save_image_file("trait_adb_screenshot.png", &image).unwrap();

    println!("Execute complete, please check the image file.");
}

#[test]
fn xcap_wait_template() {
    let monitors = xcap::Monitor::all().unwrap();
    let monitor = monitors.into_iter().next().unwrap();
    let xcap_screenshoter = XcapScreenshoter::new(monitor);

    let template = cv::load_image_file("test_assets/image_wait_template.png").unwrap();
    xcap_screenshoter
        .wait_template_existence(&template, 0.9f32, 1f32)
        .unwrap();
}

#[test]
fn adb_wait_template() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let adb_screenshoter = AdbScreenshoter::default();

    let template = cv::load_image_file("test_assets/image_wait_template.png").unwrap();
    adb_screenshoter
        .wait_template_existence(&template, 0.9f32, 1f32)
        .unwrap();
}
