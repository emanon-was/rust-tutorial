#[derive(Debug)]
pub enum TextureSize {
    W512, // 512 x 512
}

impl TextureSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            TextureSize::W512 => (512, 512),
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub size: TextureSize,
    pub x: u32,
    pub y: u32,
    pub path: String,
}

impl Texture {
    pub fn w512(x: u32, y: u32, path: &str) -> Texture {
        Texture {
            size: TextureSize::W512,
            x: x,
            y: y,
            path: path.to_owned(),
        }
    }
}

impl Texture {
    pub fn dimensions(&self) -> (u32, u32) {
        self.size.dimensions()
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
        let t = Texture::w512(2, 3, "test.png");
        let (x, y) = t.offset();
        assert!(x == 1024);
        assert!(y == 1536);
    }

}
