use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub fn point(framebuffer: &mut Framebuffer, position: Vector2) {
    let x = position.x as i32;
    let y = position.y as i32;

    if x < 0 || y < 0 || x >= framebuffer.width() || y >= framebuffer.height() {
        return;
    }

    let color = framebuffer.get_current_color();

    framebuffer.color_buffer.draw_pixel(x, y, color);
}
