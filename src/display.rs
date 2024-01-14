use console::{Style, Term};
use crate::process::Test;

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
pub struct Display {
    term: Term,
}

impl Display {

    pub fn new(term: Term) -> Self {
        Self {
            term,
        }
    }

    pub fn print_tests(&self, tests: &Vec<Test>) {

        let tests_passed = tests.iter().filter(|t| t.success).count();
        let tests_failed = tests.len() - tests_passed;

        self.term.write_line(&format!(
            "  {} tests passed, {} tests failed",
            get_color(Color::Green, Some(TextStyle::Bold)).apply_to(tests_passed),
            get_color(Color::Red, Some(TextStyle::Bold)).apply_to(tests_failed),
        )).ok();

        for test in tests {
            self.term.write_line(&format!(
                "    {} {}",
                get_color(Color::Green, Some(TextStyle::Bold)).apply_to("PASS"),
                get_color(Color::White, Some(TextStyle::Bold)).apply_to(format!("{}({})", test.config.endpoint_name.clone(), test.config.endpoint_url.clone())),
            )).ok();
        }
    }

}
