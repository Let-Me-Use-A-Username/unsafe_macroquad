use crate::math::{vec2, Rect, Vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Circle {
    /// Constructs a new `Circle` with the center `(x, y)` and radius `r`.
    pub const fn new(x: f32, y: f32, r: f32) -> Self {
        Circle { x, y, r }
    }

    /// Returns the center point of the `Circle`.
    pub const fn point(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Returns the radius of the `Circle`.
    pub const fn radius(&self) -> f32 {
        self.r
    }

    /// Moves the `Circle`'s origin to (x, y).
    pub const fn move_to(&mut self, destination: Vec2) {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Circle` by a factor of `sr`.
    pub const fn scale(&mut self, sr: f32) {
        self.r *= sr;
    }

    /// Checks whether the `Circle` contains a `Point`.
    pub fn contains(&self, pos: &Vec2) -> bool {
        pos.distance(vec2(self.x, self.y)) < self.r
    }

    /// Checks whether the `Circle` overlaps a `Circle`.
    pub fn overlaps(&self, other: &Circle) -> bool {
        self.point().distance(other.point()) < self.r + other.r
    }

    /// Translate `Circle` origin by `offset` vector.
    pub const fn offset(self, offset: Vec2) -> Circle {
        Circle::new(self.x + offset.x, self.y + offset.y, self.r)
    }
}
