use stylize::{self, Modifier, StylizedString, Modifier::*, Color::*};

use stylize::Stylizer;

fn main() {
    let first_name: String = String::from("joe");
    let first_name_stylized = StylizedString {
        text: first_name,
        modifiers: vec![
            Bold,
            Italic,
        ],
        color: BlackBright,
        background_color: RGB(100,55,200),
    };

    let last_name: String = String::from("Gesualdo");
    let last_name_stylized = StylizedString {
        text: last_name,
        modifiers: vec![
            Bold,
            Italic,
        ],
        color: Green,
        background_color: RGB(5,55,100),
    };

    println!(
        "{} {}",
        first_name_stylized.to_string(),
        last_name_stylized.to_string(),
    );
}

