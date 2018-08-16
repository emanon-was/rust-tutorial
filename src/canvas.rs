extern crate image;

use self::image::{GenericImage, ImageBuffer};
use texture::Texture;

pub struct Canvas {
    size: CanvasSize,
}

#[allow(dead_code)]
pub enum CanvasSize {
    W1024, // 1024 x 1311
    W2048, // 2048 x 2622
}

impl CanvasSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            CanvasSize::W1024 => (1024, 1311),
            CanvasSize::W2048 => (2048, 2622),
        }
    }
}

#[allow(dead_code)]
impl Canvas {
    pub fn w1024() -> Canvas {
        let size = CanvasSize::W1024;
        Canvas { size: size }
    }
    pub fn w2048() -> Canvas {
        let size = CanvasSize::W2048;
        Canvas { size: size }
    }
}

impl Canvas {
    #[allow(unused_parens)]
    pub fn write(&self, path: &str, textures: Vec<Texture>) -> () {
        let (max_x, max_y) = self.size.dimensions();
        let mut img = ImageBuffer::new(max_x, max_y);
        for t in &textures {
            let (offset_x, offset_y) = t.offset();
            let subimg = image::open(t.path.to_owned()).unwrap();
            let (sub_x, sub_y) = t.dimensions();
            for x in 0..sub_x {
                for y in 0..sub_y {
                    let (pos_x, pos_y) = (x + offset_x, y + offset_y);
                    if (pos_x < max_x && pos_y < max_y) {
                        let pixel = subimg.get_pixel(x, y);
                        img.put_pixel(pos_x, pos_y, pixel);
                    }
                }
            }
        }
        img.save(path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let s = Canvas::w1024();
        let (x, y) = s.size.dimensions();
        assert!(x == 1024);
        assert!(y == 1311);
    }

    #[test]
    fn write() {
        let s = Canvas::w1024();
        s.write(
            "test.jpg",
            vec![
                Texture::default(0, 0, "testdata/texture-99-1-0-0.jpg"),
                Texture::default(0, 1, "testdata/texture-99-1-0-1.jpg"),
                Texture::default(0, 2, "testdata/texture-99-1-0-2.jpg"),
                Texture::default(1, 0, "testdata/texture-99-1-1-0.jpg"),
                Texture::default(1, 1, "testdata/texture-99-1-1-1.jpg"),
                Texture::default(1, 2, "testdata/texture-99-1-1-2.jpg"),
            ],
        );
    }

}
