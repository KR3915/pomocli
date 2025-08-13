use figlet_rs::FIGfont;
use std::{thread, time};
use std::io::{stdout, Write};
use terminal_size::{Width, Height, terminal_size};
use notify_rust::Notification;

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

fn timer_loop(
    minutes: i32,
    seconds: i32,
    standard_font: &FIGfont,
    timer_message: &str,
    finished_message: &str,
) {
    let mut minutes = minutes;
    let mut seconds = seconds;
    clear_screen();

    loop {
        let time = format!("{}:{:02}", minutes, seconds);
        let figure = standard_font.convert(&time);
        if let Some(ref figure) = figure {
            print_centered_figure(figure);
            print_centered_text(timer_message);
        }
        thread::sleep(time::Duration::from_secs(1));
        if seconds == 0 {
            if minutes == 0 {
                clear_screen();
                print_centered_text(finished_message);

                // Send notification
                let _ = Notification::new()
                    .summary("Pomodoro Timer")
                    .body(finished_message)
                    .show();

                thread::sleep(time::Duration::from_secs(2));
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

fn main() {
    let mut is_break: bool = false;
    let mut count = 0;
    let standard_font = FIGfont::standard().unwrap();

    loop {
        if count == 4 {
            count = 0;
            timer_loop(
                0,
                5,
                &standard_font,
                "LONG BREAK >.<!!",
                "GET BACK TO WORK!!!!",
            );
            is_break = false;
            continue;
        }
        if is_break {
            timer_loop(
                0,
                5,
                &standard_font,
                "BREAK TIME >.<",
                "GET YO ASS BACK TO WORK!!!",
            );
            is_break = false;
        } else {
            timer_loop(
                0,
                5,
                &standard_font,
                "TIME TO WORK :(",
                "time to relax ＾･ｪ･＾",
            );
            is_break = true;
            count += 1;
        }
    }
}



