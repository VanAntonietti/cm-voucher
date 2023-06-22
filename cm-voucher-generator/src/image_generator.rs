use imageproc::drawing::draw_text_mut;
use image::{DynamicImage, Rgba};
use rusttype::{Font, Scale};

pub fn image_generator() {
    let mut img = image::open("../assets/template.png").unwrap();
    let font = Font::try_from_bytes(include_bytes!("../assets/birdsofparadise.ttf")).unwrap();
    let scale = Scale { x: 100.0, y: 100.0 };
    let text = data.name;
    let x = 100;
    let y = 100;
    draw_text_mut(&mut img, Rgba([255, 255, 255, 255]), x, y, scale, &font, &name);
    img.save("~/Downloads/test.png").unwrap();
    }