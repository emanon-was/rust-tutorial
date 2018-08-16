mod texture;
use texture::Texture;
mod canvas;
use canvas::Canvas;

fn main() {
    let s = Canvas::w2048();
    s.write(
        "test.jpg",
        vec![
            Texture::w512(0, 0, "testdata/texture-99-2-0-0.jpg"),
            Texture::w512(0, 1, "testdata/texture-99-2-0-1.jpg"),
            Texture::w512(0, 2, "testdata/texture-99-2-0-2.jpg"),
            Texture::w512(0, 3, "testdata/texture-99-2-0-3.jpg"),
            Texture::w512(0, 4, "testdata/texture-99-2-0-4.jpg"),
            Texture::w512(0, 5, "testdata/texture-99-2-0-5.jpg"),
            Texture::w512(1, 0, "testdata/texture-99-2-1-0.jpg"),
            Texture::w512(1, 1, "testdata/texture-99-2-1-1.jpg"),
            Texture::w512(1, 2, "testdata/texture-99-2-1-2.jpg"),
            Texture::w512(1, 3, "testdata/texture-99-2-1-3.jpg"),
            Texture::w512(1, 4, "testdata/texture-99-2-1-4.jpg"),
            Texture::w512(1, 5, "testdata/texture-99-2-1-5.jpg"),
            Texture::w512(2, 0, "testdata/texture-99-2-2-0.jpg"),
            Texture::w512(2, 1, "testdata/texture-99-2-2-1.jpg"),
            Texture::w512(2, 2, "testdata/texture-99-2-2-2.jpg"),
            Texture::w512(2, 3, "testdata/texture-99-2-2-3.jpg"),
            Texture::w512(2, 4, "testdata/texture-99-2-2-4.jpg"),
            Texture::w512(2, 5, "testdata/texture-99-2-2-5.jpg"),
            Texture::w512(3, 0, "testdata/texture-99-2-3-0.jpg"),
            Texture::w512(3, 1, "testdata/texture-99-2-3-1.jpg"),
            Texture::w512(3, 2, "testdata/texture-99-2-3-2.jpg"),
            Texture::w512(3, 3, "testdata/texture-99-2-3-3.jpg"),
            Texture::w512(3, 4, "testdata/texture-99-2-3-4.jpg"),
            Texture::w512(3, 5, "testdata/texture-99-2-3-5.jpg"),
        ],
    );
}
