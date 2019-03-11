pub mod prelude {
    pub use super::Rect;
    pub use super::RectBuilder;
}

pub struct Rect {
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
}

#[derive(Default)]
pub struct RectBuilder {
    top:    f32,
    bottom: f32,
    left:   f32,
    right:  f32,
}

impl RectBuilder {
    pub fn new() -> Self {
        RectBuilder::default()
    }

    pub fn top(mut self, pos: f32) -> Self {
        self.top = pos;
        self
    }
    pub fn bottom(mut self, pos: f32) -> Self {
        self.bottom = pos;
        self
    }
    pub fn left(mut self, pos: f32) -> Self {
        self.left = pos;
        self
    }
    pub fn right(mut self, pos: f32) -> Self {
        self.right = pos;
        self
    }

    pub fn build(self) -> Rect {
        Rect {
            top:    self.top,
            bottom: self.bottom,
            left:   self.left,
            right:  self.right,
        }
    }
}

impl From<&Rect> for RectBuilder {
    fn from(rect: &Rect) -> RectBuilder {
        RectBuilder {
            top:    rect.top,
            bottom: rect.bottom,
            left:   rect.left,
            right:  rect.right,
        }
    }
}
