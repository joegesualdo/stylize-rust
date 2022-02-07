use ansi_escape_codes::{
    BackgroundColor, BrightBackgroundColor, BrightForegroundColor, Color::*, EscapeSequence,
    ForegroundColor, SelectGraphicRenditionParameter,
};

pub enum Modifier {
    Bold,
    Italic,
    Underline,
}
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,
    Default,
    RGB(u32, u32, u32),
}

pub struct StylizedString {
    pub text: String,
    pub modifiers: Vec<Modifier>,
    pub color: Color,
    pub background_color: Color,
}

impl StylizedString {
    pub fn to_string(self) -> String {
        let mut params: Vec<SelectGraphicRenditionParameter> = self
            .modifiers
            .into_iter()
            .map(|modifier| match modifier {
                Modifier::Bold => SelectGraphicRenditionParameter::BoldParameter,
                Modifier::Italic => SelectGraphicRenditionParameter::ItalicParameter,
                Modifier::Underline => SelectGraphicRenditionParameter::UnderlineParameter,
            })
            .collect();
        let foreground_color_param = match self.color {
            Color::Black => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::BlackForeground,
            ),
            Color::Red => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::RedForeground,
            ),
            Color::Green => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::GreenForeground,
            ),
            Color::Yellow => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::YellowForeground,
            ),
            Color::Blue => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::BlueForeground,
            ),
            Color::Magenta => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::MagentaForeground,
            ),
            Color::Cyan => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::CyanForeground,
            ),
            Color::White => SelectGraphicRenditionParameter::SetForegroundColorParameter(
                ForegroundColor::WhiteForeground,
            ),
            Color::BlackBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightBlackForeground,
                )
            }
            Color::RedBright => SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                BrightForegroundColor::BrightRedForeground,
            ),
            Color::GreenBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightGreenForeground,
                )
            }
            Color::YellowBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightYellowForeground,
                )
            }
            Color::BlueBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightBlueForeground,
                )
            }
            Color::MagentaBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightMagentaForeground,
                )
            }
            Color::CyanBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightCyanForeground,
                )
            }
            Color::WhiteBright => {
                SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                    BrightForegroundColor::BrightWhiteForeground,
                )
            }
            Color::Default => SelectGraphicRenditionParameter::DefaultForegroundColorParameter,
            Color::RGB(red, green, blue) => {
                SelectGraphicRenditionParameter::SetForegroundColorParameter(
                    ForegroundColor::ForegroundColor(ansi_escape_codes::Color::RGB(
                        red, green, blue,
                    )),
                )
            }
        };
        let background_color_param = match self.background_color {
            Color::Black => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::BlackBackground,
            ),
            Color::Red => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::RedBackground,
            ),
            Color::Green => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::GreenBackground,
            ),
            Color::Yellow => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::YellowBackground,
            ),
            Color::Blue => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::BlueBackground,
            ),
            Color::Magenta => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::MagentaBackground,
            ),
            Color::Cyan => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::CyanBackground,
            ),
            Color::White => SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                BackgroundColor::WhiteBackground,
            ),
            Color::BlackBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightBlackBackground,
                )
            }
            Color::RedBright => SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                BrightBackgroundColor::BrightRedBackground,
            ),
            Color::GreenBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightGreenBackground,
                )
            }
            Color::YellowBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightYellowBackground,
                )
            }
            Color::BlueBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightBlueBackground,
                )
            }
            Color::MagentaBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightMagentaBackground,
                )
            }
            Color::CyanBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightCyanBackground,
                )
            }
            Color::WhiteBright => {
                SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                    BrightBackgroundColor::BrightWhiteBackground,
                )
            }
            Color::RGB(red, green, blue) => {
                SelectGraphicRenditionParameter::SetBackgroundColorParameter(
                    BackgroundColor::BackgroundColor(ansi_escape_codes::Color::RGB(
                        red, green, blue,
                    )),
                )
            }
            Color::Default => SelectGraphicRenditionParameter::DefaultBackgroundColorParameter,
        };
        params.push(foreground_color_param);
        params.push(background_color_param);
        let s = format!(
            "{}{}{}",
            EscapeSequence::SelectGraphicRenditionSequence(params).to_string(),
            self.text,
            EscapeSequence::ResetColorsSequence.to_string()
        );
        return s;
    }
}

pub struct Stylizer {
    text: String,
    color: ForegroundColor,
}

impl Stylizer {
    pub fn new(text: String) -> Self {
        Stylizer {
            text: text,
            color: ForegroundColor::ForegroundColor(Color256(200)),
        }
    }
    pub fn blue(&mut self) {
        self.color = ForegroundColor::BlueForeground;
        //return self
    }
    pub fn to_string(self) -> String {
        let params: Vec<SelectGraphicRenditionParameter> =
            vec![SelectGraphicRenditionParameter::SetForegroundColorParameter(self.color)];
        let s = format!(
            "{}{}{}",
            EscapeSequence::SelectGraphicRenditionSequence(params).to_string(),
            self.text,
            EscapeSequence::ResetColorsSequence.to_string()
        );
        return s;
    }
}
pub fn blue(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Blue,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}

pub fn red(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Red,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}

pub fn black(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Black,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
pub fn green(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Green,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
pub fn yellow(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Yellow,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
pub fn magenta(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Magenta,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
pub fn cyan(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::Cyan,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
pub fn white(text: String) -> String {
    let stylized_string = StylizedString {
        text,
        modifiers: vec![],
        color: Color::White,
        background_color: Color::Default,
    };
    return stylized_string.to_string();
}
