use crate::Models::ColorRGB;
pub struct Render {
    x: u32,
    y: u32,
    h: u32,
    w: u32,
    color: Option<ColorRGB>,
    texture: Option<String>,
}

impl Render {
    fn new(
        x: u32,
        y: u32,
        h: u32,
        w: u32,
        color: Option<(u8, u8, u8)>,
        texture: Option<String>,
    ) -> Self {
        Render {
            x,
            y,
            w,
            h,
            color,
            texture,
        }
    }
}

pub trait RenderTrait: Send + Sync {
    fn into_renderable(&self) -> Render;
    fn next_frame(&mut self);
}
