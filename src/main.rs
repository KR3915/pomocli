use figlet_rs::FIGfont;
use std::{thread, time};
use std::io::{stdout, Write};
use terminal_size::{Width, Height, terminal_size};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use json_comments::StripComments;

#[derive(Deserialize)]
struct Config {
    work_minutes: u32,
    break_minutes: u32,
    long_break_minutes: u32,
    pomodoros_before_long_break: u32,
    work_message: String,
    break_message: String,
    long_break_message: String,
}

const DEFAULT_CONFIG: &str = r#"{
  // Pomodoro work duration in minutes
  "work_minutes": 25,
  // Short break duration in minutes
  "break_minutes": 5,
  // Long break duration in minutes
  "long_break_minutes": 15,
  // Number of pomodoros before a long break
  "pomodoros_before_long_break": 4,
  // Messages
  "work_message": "TIME TO WORK :(",
  "break_message": "BREAK TIME >.<",
  "long_break_message": "LONG BREAK >.<!!"
}
"#;

fn load_config() -> Config {
    let mut config_path = dirs::home_dir().unwrap_or(PathBuf::from("/"));
    config_path.push(".config/pomocli/config.jsonc");
    let config_str = fs::read_to_string(&config_path)
        .expect("Failed to read config file");
    let stripped = StripComments::new(config_str.as_bytes());
    serde_json::from_reader(stripped)
        .expect("Failed to parse config file")
}

fn ensure_config_exists() {
    let mut config_path = dirs::home_dir().unwrap_or(PathBuf::from("/"));
    config_path.push(".config/pomocli/config.jsonc");
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create config directory");
        }
        std::fs::write(&config_path, DEFAULT_CONFIG).expect("Failed to write default config");
        println!("Created default config at {:?}", config_path);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

// Center FIGlet output horizontally and vertically
fn print_centered_figure(figure: &figlet_rs::FIGure) {
    if let Some((Width(width), Height(height))) = terminal_size() {
        let figlet_string = figure.to_string();
        let lines: Vec<&str> = figlet_string.lines().collect();
        let figlet_height = lines.len();
        let vertical_padding = (height as usize).saturating_sub(figlet_height) / 2;

        // Print vertical padding (blank lines)
        for _ in 0..vertical_padding {
            println!();
        }

        // Print each line centered horizontally
        for line in lines {
            let padding = (width as usize).saturating_sub(line.len()) / 2;
            println!("{:padding$}{}", "", line, padding = padding);
        }
    } else {
        // fallback: just print
        println!("{}", figure);
    }
}

// Add this function for centering plain text
fn print_centered_text(text: &str) {
    if let Some((Width(width), Height(height))) = terminal_size() {
        let lines: Vec<&str> = text.lines().collect();
        let text_height = lines.len();
        let vertical_padding = (height as usize).saturating_sub(text_height) / 2;

        for _ in 0..vertical_padding {
            println!();
        }
        for line in lines {
            let padding = (width as usize).saturating_sub(line.len()) / 2;
            println!("{:padding$}{}", "", line, padding = padding);
        }
    } else {
        println!("{}", text);
    }
}

fn main() {
    ensure_config_exists();
    let config = load_config();
    let mut is_break: bool = false;
    let mut count = 0;

    let standard_font = FIGfont::standard().unwrap();

    loop {
        // Long break
        if count == config.pomodoros_before_long_break {
            count = 0;
            let mut minutes = config.long_break_minutes as i32;
            let mut seconds = 0;
            clear_screen();
            loop {
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure {
                    print_centered_figure(figure);
                    print_centered_text(&config.long_break_message);
                }
                thread::sleep(time::Duration::from_secs(1));
                if seconds == 0 {
                    if minutes == 0 {
                        is_break = false;
                        break;
                    } else {
                        minutes -= 1;
                        seconds = 59;
                    }
                } else {
                    seconds -= 1;
                }
                clear_screen();
            }
        }
        // Break
        else if is_break {
            let mut minutes = config.break_minutes as i32;
            let mut seconds = 0;
            clear_screen();
            loop {
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure {
                    print_centered_figure(figure);
                    print_centered_text(&config.break_message);
                }
                thread::sleep(time::Duration::from_secs(1));
                if seconds == 0 {
                    if minutes == 0 {
                        is_break = false;
                        break;
                    } else {
                        minutes -= 1;
                        seconds = 59;
                    }
                } else {
                    seconds -= 1;
                }
                clear_screen();
            }
        }
        // Work
        else {
            let mut minutes = config.work_minutes as i32;
            let mut seconds = 0;
            clear_screen();
            loop {
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure {
                    print_centered_figure(figure);
                    print_centered_text(&config.work_message);
                }
                thread::sleep(time::Duration::from_secs(1));
                if seconds == 0 {
                    if minutes == 0 {
                        is_break = true;
                        count += 1;
                        break;
                    } else {
                        minutes -= 1;
                        seconds = 59;
                    }
                } else {
                    seconds -= 1;
                }
                clear_screen();
            }
        }
    }
}



