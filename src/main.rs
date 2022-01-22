use std::path::Path;

use image::imageops::{self, FilterType};
use image::{self, GenericImage, Rgb, RgbImage};
use pyxel::{Pyxel, PyxelCallback};

const BOX_X1: f64 = 15.0;
const BOX_X2: f64 = 90.0;
const BOX_SIZE: f64 = 5.0;
const LABEL_X1: f64 = 34.0;
const LABEL_X2: f64 = 95.0;
const LABEL_Y: f64 = 5.0;
const ITEM_X1: f64 = 25.0;
const ITEM_X2: f64 = 100.0;
const ITEM_Y1: f64 = 15.0;
const ITEM_Y2: f64 = 25.0;
const ITEM_Y3: f64 = 35.0;
const BG_COLOR: pyxel::Color = 7;
const TEXT_COLOR: pyxel::Color = 0;
const SELECT_COLOR: pyxel::Color = 8;

pub struct App {
    size_type: u32,
    color_type: u32,
}

impl App {
    fn init() {
        let mut pyxel = Pyxel::new(140, 140, Some("gobliso"), None, None, None, None);
        pyxel.mouse(true);
        let mut app = App {
            size_type: 0,
            color_type: 0,
        };
        pyxel.run(&mut app);
    }

    fn resize_image(&self, filename: &str) {
        let src_filename = Path::new(&filename);
        let src_image = image::open(&src_filename).unwrap().to_rgb8();
        let (src_width, src_height) = src_image.dimensions();
        let (dst_width, dst_height, prefix) = match self.size_type {
            0 => (1080, 1080, "_1080x1080_"),
            1 => (1080, 566, "_1080x566_"),
            2 => (1080, 1350, "_1080x1350_"),
            _ => panic!(),
        };
        let dst_filename = src_filename
            .with_file_name(
                prefix.to_string() + src_filename.file_name().unwrap().to_str().unwrap(),
            )
            .with_extension("jpg");
        let color = match self.color_type {
            0 => Rgb([255, 255, 255]),
            1 => Rgb([0, 0, 0]),
            _ => panic!(),
        };
        let mut dst_image = RgbImage::new(dst_width, dst_height);
        for y in 0..dst_height {
            for x in 0..dst_width {
                dst_image.put_pixel(x, y, color);
            }
        }
        let scale = f64::min(
            dst_width as f64 / src_width as f64,
            dst_height as f64 / src_height as f64,
        );
        let scaled_width = (src_width as f64 * scale) as u32;
        let scaled_height = (src_height as f64 * scale) as u32;
        let scaled_image = imageops::resize(
            &src_image,
            scaled_width,
            scaled_height,
            FilterType::Lanczos3,
        );
        dst_image
            .copy_from(
                &scaled_image,
                (dst_width - scaled_width) / 2,
                (dst_height - scaled_height) / 2,
            )
            .unwrap();
        dst_image.save(dst_filename).unwrap();
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.btnp(pyxel::MOUSE_BUTTON_LEFT, None, None) {
            let mouse_x = pyxel.mouse_x() as f64;
            let mouse_y = pyxel.mouse_y() as f64;
            for i in 0..3 {
                let item_y = ITEM_Y1 + i as f64 * 10.0;
                if mouse_y < item_y - 2.0 || mouse_y >= item_y + 7.0 {
                    continue;
                }
                if (BOX_X1 - 2.0..ITEM_X1 + 46.0).contains(&mouse_x) {
                    self.size_type = i;
                } else if i < 2 && (BOX_X2 - 2.0..ITEM_X2 + 22.0).contains(&mouse_x) {
                    self.color_type = i;
                }
            }
        }
        for filename in pyxel.drop_files() {
            self.resize_image(filename);
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(BG_COLOR);

        pyxel.text(LABEL_X1, LABEL_Y, "SIZE", TEXT_COLOR);
        pyxel.rectb(BOX_X1, ITEM_Y1, BOX_SIZE, BOX_SIZE, TEXT_COLOR);
        pyxel.text(ITEM_X1, ITEM_Y1, "1080 x 1080", TEXT_COLOR);
        pyxel.rectb(BOX_X1, ITEM_Y2, BOX_SIZE, BOX_SIZE, TEXT_COLOR);
        pyxel.text(ITEM_X1, ITEM_Y2, "1080 x  566", TEXT_COLOR);
        pyxel.rectb(BOX_X1, ITEM_Y3, BOX_SIZE, BOX_SIZE, TEXT_COLOR);
        pyxel.text(ITEM_X1, ITEM_Y3, "1080 x 1350", TEXT_COLOR);
        pyxel.rect(
            BOX_X1 + 1.0,
            ITEM_Y1 + self.size_type as f64 * 10.0 + 1.0,
            BOX_SIZE - 2.0,
            BOX_SIZE - 2.0,
            SELECT_COLOR,
        );

        pyxel.text(LABEL_X2, LABEL_Y, "COLOR", TEXT_COLOR);
        pyxel.text(ITEM_X2, ITEM_Y1, "WHITE", TEXT_COLOR);
        pyxel.rectb(BOX_X2, ITEM_Y1, BOX_SIZE, BOX_SIZE, TEXT_COLOR);
        pyxel.text(ITEM_X2, ITEM_Y2, "BLACK", TEXT_COLOR);
        pyxel.rectb(BOX_X2, ITEM_Y2, BOX_SIZE, BOX_SIZE, TEXT_COLOR);
        pyxel.rect(
            BOX_X2 + 1.0,
            ITEM_Y1 + self.color_type as f64 * 10.0 + 1.0,
            BOX_SIZE - 2.0,
            BOX_SIZE - 2.0,
            SELECT_COLOR,
        );

        pyxel.rectb(10.0, 45.0, 120.0, 90.0, TEXT_COLOR);
        pyxel.text(37.0, 87.0, "DROP IMAGES HERE!", TEXT_COLOR);
    }
}

pub fn main() {
    App::init();
}
