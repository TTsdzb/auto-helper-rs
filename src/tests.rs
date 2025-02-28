use std::{thread, time};

use crate::{
    adb_commands, cv,
    inputer::{EnigoInputer, Inputer},
    screenshoter::{AdbScreenshoter, Screenshoter, XcapScreenshoter},
    structs::Point,
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
fn cv_match_template() {
    let source = cv::load_image_file("test_assets/image_source.png").unwrap();
    let template = cv::load_image_file("test_assets/image_template.png").unwrap();

    let res = cv::cv_match_template_center(&source, &template);

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
    let interval = time::Duration::from_secs(1);
    xcap_screenshoter
        .wait_template_existence(&template, 0.9f32, interval)
        .unwrap();
}

#[test]
fn adb_wait_template() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let adb_screenshoter = AdbScreenshoter::default();

    let template = cv::load_image_file("test_assets/image_wait_template.png").unwrap();
    let interval = time::Duration::from_secs(1);
    adb_screenshoter
        .wait_template_existence(&template, 0.9f32, interval)
        .unwrap();
}

#[test]
fn trait_enigo_click() {
    let mut inputer = EnigoInputer::default().unwrap();

    thread::sleep(time::Duration::from_millis(500));

    inputer.click(&Point::new(30, 1050)).unwrap();
}
