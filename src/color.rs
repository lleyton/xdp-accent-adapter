use zvariant::{OwnedValue, Type};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Type, OwnedValue)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn to_hex_code(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }
}
