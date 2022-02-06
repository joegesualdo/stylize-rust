use stylize;
use stylize::{Color::*, Modifier, Modifier::*, StylizedString};

fn stylize_name(first: String, last: String) -> String {
    let first_name: String = String::from(first);
    let first_name_stylized = StylizedString {
        text: first_name,
        modifiers: vec![Bold, Italic],
        color: Green,
        background_color: RGB(5, 55, 100),
    };

    let last_name: String = String::from(last);
    let last_name_stylized = StylizedString {
        text: last_name,
        modifiers: vec![Bold, Italic],
        color: RGB(242, 169, 0),
        background_color: RGB(77, 77, 78),
    };
    format!(
        "{} {}",
        first_name_stylized.to_string(),
        last_name_stylized.to_string(),
    )
}

fn main() {
    println!(
        "{}",
        stylize_name(String::from("Peter"), String::from("Bitcoin!"),)
    );
    println!("{}", stylize::blue(String::from("blue")));
    println!("{}", stylize::red(String::from("red")));
    println!("{}", stylize::black(String::from("black")));
    println!("{}", stylize::green(String::from("green")));
    println!("{}", stylize::yellow(String::from("yellow")));
    println!("{}", stylize::magenta(String::from("magenta")));
    println!("{}", stylize::cyan(String::from("cyan")));
    println!("{}", stylize::white(String::from("white")));
}
