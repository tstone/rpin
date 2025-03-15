use colors_transform::{Color, Hsl};

pub trait HslExt {
    fn to_hex(&self) -> String;
}

impl HslExt for Hsl {
    fn to_hex(&self) -> String {
        let binding = self.to_rgb().to_css_hex_string();
        let mut chars = binding.chars();
        chars.next();
        return chars.collect::<String>();
    }
}
