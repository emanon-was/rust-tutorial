#[derive(Debug)]
pub struct Texture {
    pub size: TextureSize,
    pub x: u32,
    pub y: u32,
    pub path: String,
}

#[derive(Debug)]
pub enum TextureSize {
    Default, // 512 x 512
}

impl Texture {
    pub fn default(x: u32, y: u32, path: &str) -> Texture {
        Texture {
            size: TextureSize::Default,
            x: x,
            y: y,
            path: path.to_owned(),
        }
    }
}

impl Texture {
    pub fn dimensions(&self) -> (u32, u32) {
        match self.size {
            TextureSize::Default => (512, 512),
        }
    }
    pub fn offset(&self) -> (u32, u32) {
        let (width, height) = self.dimensions();
        (width * self.x, height * self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset() {
        let t = Texture::default(2, 3, "test.png");
        let (x, y) = t.offset();
        assert!(x == 1024);
        assert!(y == 1536);
    }

}
