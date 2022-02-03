use stylize;

fn main() {
    let name: String = String::from("joe");
    println!("{}", name);
    println!(
        "{}",
        stylize::blue("This is blue!".to_string())
    );
    println!(
        "{}",
        stylize::red("This is red!".to_string())
    );
}
