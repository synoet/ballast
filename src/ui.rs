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

pub fn print_time_title(term: &Term, name: &str, url: &str) {
    term.write_line(&format!(
        "  {}: [{}]",
        get_color(Color::Green, Some(TextStyle::Bold)).apply_to(name),
        get_color(Color::White, Some(TextStyle::Bold)).apply_to(url)
    ))
    .ok();
}

pub fn print_time_stat(term: &Term, new_time: &f64, old_time: Option<&f64>, description: &str) {
    let diff_color = match old_time {
        Some(old_time) => match (new_time - old_time) > 0.0 {
            true => get_color(Color::Red, None),
            false => get_color(Color::Green, None),
        },
        None => get_color(Color::White, None),
    };

    let diff_string = match old_time {
        Some(old_time) => format!(
            "({}{:.2}ms)",
            get_sign_string(new_time - old_time),
            new_time - old_time
        ),
        None => "".to_string(),
    };

    term.write_line(&format!(
        "    {} {} {}",
        get_color(Color::White, None).apply_to(description),
        get_color(Color::White, None).apply_to(format!("{:.2}ms", new_time)),
        diff_color.apply_to(diff_string)
    ))
    .ok();
}

pub fn print_endpoint_in_progress(term: &Term, url: &str, total_cycles: u64, counter: u64) {
    if counter > 0 {
        term.clear_last_lines(1).ok();
    }
    let spinner_frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    term.write_line(&format!(
        "  {} Cycle [{}/{}] for {}",
        get_color(Color::Yellow, None).apply_to(spinner_frames[counter as usize % spinner_frames.len()]),
        counter + 1,
        total_cycles,
        get_color(Color::White, None).apply_to(url)
    ))
    .ok();
}

pub fn print_endpoint_finished(term: &Term, total_cycles: u64, url: &str) {
    term.clear_last_lines(1).ok();
        term.write_line(&format!(
            "  {} Cycle [{}/{}] for {}",
            get_color(Color::Green, None).apply_to("✓"),
            total_cycles,
            total_cycles,
            get_color(Color::White, None).apply_to(url)
        ))
        .ok();
}
