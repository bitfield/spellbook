use anstyle::{AnsiColor, Color, Style};

fn main() {
    print_with_color("Hello, world!", None);
}

fn print_with_color(text: &str, color: Option<AnsiColor>) {
    let color = color.unwrap_or(AnsiColor::White);
    let style = Style::new().fg_color(Some(Color::Ansi(color)));
    println!("{style}{text}{style:#}");
}
