#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus_desktop::{Config, WindowBuilder, tao::window::Icon};

use std::io::Cursor;
use image::{io::Reader as ImageReader, GenericImageView, Pixel, GenericImage, ImageBuffer};

mod ui;

struct LogoImage {
    pixels: Vec<u8>,
    width: u32,
    height: u32
}


fn load_icon() -> Option<LogoImage> {


    // let img_dyn = image::open("./src/logo.png").expect("Couldn't find 'logo.png' in root directory!");
    // let (w, h) = img_dyn.dimensions();
    // let img_buf = ImageBuffer::new(w, h);
    // let pixel_vec: Vec<u8> = Vec::new();
    // println!("{0}", imgBuf.data);
    // // for (x, y, pixel) in img.pixels() {

    // // }
    let mut img = image::open("./src/logo.png").unwrap();
    let mut imgVec: Vec<u8> = Vec::new();
    for p in img.pixels() { 
        imgVec.extend_from_slice(&p.2.0)
    }
    println!("{imgVec:?}");
    Some(LogoImage { pixels: imgVec, width: img.dimensions().0, height: img.dimensions().1 })
}

fn main() {
    let logo_image_data = match load_icon() {
        Some(img) => img,
        _ => panic!("Couldn't load logo.png image from root directory!")
    };
    
    let tao_icon = match Icon::from_rgba(
        logo_image_data.pixels,
        logo_image_data.width,
        logo_image_data.height
    ) {
        Ok(e) => Some(e),
        _ => panic!("Unable to parse loaded logo image data!"),
    };


    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        ui::App,
        Config::default().with_window(
            WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(800.0, 800.0),)
                .with_title("ImageThreader")
                .with_window_icon(tao_icon)
            )
    );
}

