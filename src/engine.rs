use sdl2::{
    gfx::framerate::FPSManager,
    image::{InitFlag, LoadTexture, Sdl2ImageContext},
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};

pub struct Game {
    pub running: bool,
    pub framerate: FPSManager,
    pub image_context: Sdl2ImageContext,
}

impl Game {
    pub fn new() -> Self {
        let mut framerate = FPSManager::new();
        framerate.set_framerate(60).unwrap();

        Self {
            running: true,
            framerate,
            image_context: sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap(),
        }
    }
}

pub struct Sprite<'a> {
    pub texture: Texture<'a>,
    pub rect: Rect,
}

impl<'a> Sprite<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        img_path: &str,
    ) -> Self {
        Self {
            texture: texture_creator.load_texture(img_path).unwrap(),
            rect: Rect::new(x, y, w, h),
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        canvas.copy(&self.texture, None, self.rect).unwrap();
    }
}

pub struct Text {
    pub content: String,
}
