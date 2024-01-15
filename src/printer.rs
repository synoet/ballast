use crate::process::Test;
use console::{Style, Term};

enum Color {
    White,
    Green,
    Yellow,
    Red,
}

enum TextStyle {
    Bold,
}

fn get_sign_string(num: f64) -> String {
    return match num >= 0.0 {
        true => "+".to_string(),
        false => "".to_string(),
    };
}

fn get_color(color: Color, text_style: Option<TextStyle>) -> Style {
    match (color, text_style) {
        (Color::White, None) => Style::new().white(),
        (Color::White, Some(TextStyle::Bold)) => Style::new().white().bold(),
        (Color::Green, None) => Style::new().green(),
        (Color::Green, Some(TextStyle::Bold)) => Style::new().green().bold(),
        (Color::Yellow, None) => Style::new().yellow(),
        (Color::Yellow, Some(TextStyle::Bold)) => Style::new().yellow().bold(),
        (Color::Red, None) => Style::new().red(),
        (Color::Red, Some(TextStyle::Bold)) => Style::new().red().bold(),
    }
}

pub struct Printer {
    term: Term,
}

impl Printer {
    pub fn new(term: Term) -> Self {
        Self { term }
    }

    pub fn print_with_green(&self, title: &str, description: &str, indent: i8) -> &Self {
        let _res = self.term.write_line(&format!(
            "{}{} {}",
            " ".repeat(indent as usize),
            get_color(Color::Green, Some(TextStyle::Bold)).apply_to(title),
            get_color(Color::White, None).apply_to(description)
        ));

        self
    }

    pub fn print_with_yellow(&self, title: &str, description: &str, indent: i8) -> &Self {
        let _res = self.term.write_line(&format!(
            "{}{} {}",
            " ".repeat(indent as usize),
            get_color(Color::Yellow, Some(TextStyle::Bold)).apply_to(title),
            get_color(Color::White, None).apply_to(description)
        ));
        self
    }

    pub fn print_with_red(&self, title: &str, description: &str, indent: i8) -> &Self {
        let _res = self.term.write_line(&format!(
            "{}{} {}",
            " ".repeat(indent as usize),
            get_color(Color::Red, Some(TextStyle::Bold)).apply_to(title),
            get_color(Color::White, None).apply_to(description)
        ));
        self
    }

    pub fn print(&self, message: &str) -> &Self {
        let _res = self.term.write_line(&format!(
            "{}",
            get_color(Color::White, None).apply_to(message)
        ));
        self
    }

    pub fn print_lines(&self, lines: &Vec<String>, indent: i8) -> &Self {
        for line in lines {
            let _res = self.term.write_line(&format!(
                "{}{}",
                " ".repeat(indent as usize),
                get_color(Color::White, None).apply_to(line)
            ));
        }
        self
    }

    pub fn print_stat(&self, title: &str, val: f64, diff: f64, unit: &str) -> &Self {
        let diff_color = match diff > 0.0 {
            true => Color::Red,
            false => Color::Green,
        };
        let _res = self.term.write_line(&format!(
            "{}{}: {} {}",
            " ".repeat(4),
            get_color(Color::White, None).apply_to(title),
            get_color(Color::White, None).apply_to(val),
            get_color(diff_color, None).apply_to(format!(
                "({}{}{})",
                get_sign_string(diff),
                diff,
                unit
            ))
        ));

        self
    }

    pub fn clear_previous(&self) -> &Self {
        self.term.clear_last_lines(1).ok();
        self
    }

    pub fn blank_line(&self) -> &Self {
        self.term.write_line("").ok();
        self
    }
}
