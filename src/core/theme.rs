use std::cmp::Ordering;
use iced::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub palette: ColorPalette,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub fn all() -> Vec<(String, Theme)> {
        vec![
            ("Dark".to_string(), Theme::dark()),
            ("Light".to_string(), Theme::light()),
        ]
    }

    pub fn dark() -> Theme {
        Theme {
            name: "Dark".to_string(),
            palette: ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#111111").unwrap(),
                    foreground: hex_to_color("#1C1C1C").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#3f2b56").unwrap(),
                    secondary: hex_to_color("#4a3c1c").unwrap(),
                    surface: hex_to_color("#828282").unwrap(),
                    error: hex_to_color("#992B2B").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#BA84FC").unwrap(),
                    secondary: hex_to_color("#ffd03c").unwrap(),
                    surface: hex_to_color("#E0E0E0").unwrap(),
                    error: hex_to_color("#C13047").unwrap(),
                },
            },
        }
    }

    pub fn light() -> Theme {
        Theme {
            name: "Light".to_string(),
            palette: ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#ffffff").unwrap(),
                    foreground: hex_to_color("#F5F5F5").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#DFDBFF").unwrap(),
                    secondary: hex_to_color("#F9D659").unwrap(),
                    surface: hex_to_color("#828282").unwrap(),
                    error: hex_to_color("#992B2B").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#9580ff").unwrap(),
                    secondary: hex_to_color("#EAA326").unwrap(),
                    surface: hex_to_color("#000000").unwrap(),
                    error: hex_to_color("#C13047").unwrap(),
                },
            },
        }
    }

    pub fn dracula() -> Theme {
        Theme {
            name: "Dracula".to_string(),
            palette: ColorPalette {
                base: BaseColors {
                    background: hex_to_color("#282a36").unwrap(),
                    foreground: hex_to_color("#353746").unwrap(),
                },
                normal: NormalColors {
                    primary: hex_to_color("#483e61").unwrap(),
                    secondary: hex_to_color("#386e50").unwrap(),
                    surface: hex_to_color("#a2a4a3").unwrap(),
                    error: hex_to_color("#A13034").unwrap(),
                },
                bright: BrightColors {
                    primary: hex_to_color("#bd94f9").unwrap(),
                    secondary: hex_to_color("#49eb7a").unwrap(),
                    surface: hex_to_color("#f4f8f3").unwrap(),
                    error: hex_to_color("#ff7ac6").unwrap(),
                },
            },
        }
    }

}

fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Theme {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Eq for Theme {}

impl Ord for Theme {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}