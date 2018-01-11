#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
pub const PURPLE: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
