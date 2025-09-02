#[derive(Clone, Copy)]
pub(super) struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub(super) fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub(super) fn interpolate(self, other: Self, f: f64) -> Self {
        let r = Self::interpolate_value(self.r, other.r, f);
        let g = Self::interpolate_value(self.g, other.g, f);
        let b = Self::interpolate_value(self.b, other.b, f);

        Self::new(r, g, b)
    }

    fn interpolate_value(a: u8, b: u8, f: f64) -> u8 {
        math::round::half_to_even(interpolate(a as f64, b as f64, f).clamp(0.0, 255.0), 0) as u8
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

fn interpolate(a: f64, b: f64, f: f64) -> f64 {
    a * (1.0 - f) + b * f
}
