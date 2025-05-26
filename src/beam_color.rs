use ratatui::style::Color;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum BeamColor {
    Prism,
    Infrared,
    Aqua,
    Emerald,
    Ice,
}

impl Default for BeamColor {
    fn default() -> Self { BeamColor::Prism }
}

impl std::str::FromStr for BeamColor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "prism" => Ok(BeamColor::Prism),
            "infrared" => Ok(BeamColor::Infrared),
            "aqua" => Ok(BeamColor::Aqua),
            "emerald" => Ok(BeamColor::Emerald),
            "ice" => Ok(BeamColor::Ice),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for BeamColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BeamColor::Prism => "Prism",
            BeamColor::Infrared => "Infrared",
            BeamColor::Aqua => "Aqua",
            BeamColor::Emerald => "Emerald",
            BeamColor::Ice => "Ice",
        };
        write!(f, "{}", s)
    }
}

impl BeamColor {
    pub fn palette(self) -> (Color, Color, Color) {
        match self {
            BeamColor::Prism => (Color::Magenta, Color::Cyan, Color::White),
            BeamColor::Infrared => (Color::Red, Color::LightRed, Color::White),
            BeamColor::Aqua => (Color::Cyan, Color::LightCyan, Color::White),
            BeamColor::Emerald => (Color::Green, Color::LightGreen, Color::White),
            BeamColor::Ice => (Color::White, Color::LightBlue, Color::White),
        }
    }
}
