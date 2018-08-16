mod texture;
use texture::Texture;
mod canvas;
use canvas::Canvas;

fn main() {
    let s = Canvas::w2048();
    s.write(
        "test.jpg",
        vec![
            Texture::default(0, 0, "testdata/texture-99-2-0-0.jpg"),
            Texture::default(0, 1, "testdata/texture-99-2-0-1.jpg"),
            Texture::default(0, 2, "testdata/texture-99-2-0-2.jpg"),
            Texture::default(0, 3, "testdata/texture-99-2-0-3.jpg"),
            Texture::default(0, 4, "testdata/texture-99-2-0-4.jpg"),
            Texture::default(0, 5, "testdata/texture-99-2-0-5.jpg"),
            Texture::default(1, 0, "testdata/texture-99-2-1-0.jpg"),
            Texture::default(1, 1, "testdata/texture-99-2-1-1.jpg"),
            Texture::default(1, 2, "testdata/texture-99-2-1-2.jpg"),
            Texture::default(1, 3, "testdata/texture-99-2-1-3.jpg"),
            Texture::default(1, 4, "testdata/texture-99-2-1-4.jpg"),
            Texture::default(1, 5, "testdata/texture-99-2-1-5.jpg"),
            Texture::default(2, 0, "testdata/texture-99-2-2-0.jpg"),
            Texture::default(2, 1, "testdata/texture-99-2-2-1.jpg"),
            Texture::default(2, 2, "testdata/texture-99-2-2-2.jpg"),
            Texture::default(2, 3, "testdata/texture-99-2-2-3.jpg"),
            Texture::default(2, 4, "testdata/texture-99-2-2-4.jpg"),
            Texture::default(2, 5, "testdata/texture-99-2-2-5.jpg"),
            Texture::default(3, 0, "testdata/texture-99-2-3-0.jpg"),
            Texture::default(3, 1, "testdata/texture-99-2-3-1.jpg"),
            Texture::default(3, 2, "testdata/texture-99-2-3-2.jpg"),
            Texture::default(3, 3, "testdata/texture-99-2-3-3.jpg"),
            Texture::default(3, 4, "testdata/texture-99-2-3-4.jpg"),
            Texture::default(3, 5, "testdata/texture-99-2-3-5.jpg"),
        ],
    );
}
