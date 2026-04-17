pub trait IntoColor {
    fn into_u32(self) -> Option<u32>;
}

// Implementazione per u32 (es. 0xFF0000)
impl IntoColor for u32 {
    fn into_u32(self) -> Option<u32> {
        Some(self)
    }
}

// Implementazione per RGB (tupla di u8)
impl IntoColor for (u8, u8, u8) {
    fn into_u32(self) -> Option<u32> {
        let (r, g, b) = self;
        Some(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
}

// Implementazione per Stringhe Hex
impl IntoColor for &str {
    fn into_u32(self) -> Option<u32> {
        let hex = self.trim_start_matches('#');
        u32::from_str_radix(hex, 16).ok()
    }
}