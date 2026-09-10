use raylib::prelude::*;

pub struct Framebuffer {
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
    width: i32,
    height: i32,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;

        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self {
            color_buffer,
            background_color,
            current_color: Color::WHITE,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_current_color(&self) -> Color {
        self.current_color
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        self.color_buffer.get_color(x, y)
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let screen_width = window.get_screen_width();
        let screen_height = window.get_screen_height();

        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);

            renderer.clear_background(Color::BLACK);

            let source = Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32);

            let destination = Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32);

            renderer.draw_texture_pro(
                &texture,
                source,
                destination,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}
