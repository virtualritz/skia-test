use exr::prelude::rgba_image::*;
use skia_safe as sk;

mod render;
use render::render_frame;

fn main() {
    let width = 256;
    let height = 256;

    let image_info = sk::ImageInfo::new(
        sk::ISize::new(width, height),
        sk::ColorType::RGBAF32,
        sk::AlphaType::Premul,
        sk::ColorSpace::new_srgb_linear().with_linear_gamma(),
    );

    let mut bitmap = sk::Bitmap::new();
    bitmap.alloc_pixels_flags(&image_info);
    let mut canvas = sk::Canvas::from_bitmap(&bitmap, None);
    //canvas.clear(sk::Color::WHITE);

    let fps = 60;
    let bpm = 60;

    let frame = 0;
    render_frame(frame, fps, bpm, &mut canvas);

    let pixmap = bitmap.pixmap();

    let sample = |position: Vec2<usize>| unsafe {
        let index = (pixmap.addr().add(position.y() * pixmap.row_bytes()) as *const f32)
            .add(position.x() << 2);
        Pixel::rgba(*index.add(0), *index.add(1), *index.add(2), *index.add(3))
    };

    let image_info = ImageInfo::rgba((width as _, height as _), SampleType::F32);

    image_info
        .write_pixels_to_file("test.exr", write_options::high(), &sample)
        .unwrap();
}
