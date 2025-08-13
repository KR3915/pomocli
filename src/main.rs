use figlet_rs::FIGfont;
use std::{thread, time};
use std::io::{stdout, Write};
use terminal_size::{Width, Height, terminal_size};

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

fn main() {
    let mut is_break: bool = false;
    let mut minutes: i32 = 0;
    let mut seconds: i32 = 0;
    let mut count = 0;

    let standard_font = FIGfont::standard().unwrap();

    loop {
        if count == 4 {
            count = 0;
            minutes = 25;
            seconds = 0;

            loop {
                clear_screen();
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

                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure {
                    print_centered_figure(figure);
                
                }
            }
        }
        if is_break {
            minutes = 5;
            seconds = 0;
        }
        loop {
            clear_screen();
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

            let time = format!("{}:{:02}", minutes, seconds);
            let figure = standard_font.convert(&time);
            if let Some(ref figure) = figure {
                print_centered_figure(figure);
            }
        }

        if !is_break {
            minutes = 20;
            seconds = 0;

            loop {
                thread::sleep(time::Duration::from_secs(1));
                clear_screen();
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

                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure {
                    print_centered_figure(figure);
                }
            }
        };
    }
}



