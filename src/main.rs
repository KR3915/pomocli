use figlet_rs::FIGfont;
use std::{thread, time};
use std::io::{stdout, Write};
fn clear_screen() 
{
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

fn main() {
    let mut is_break: bool = false;
    //minutes and seconds
    let mut minutes: i32 = 0;
    let mut seconds: i32 = 0;
    let mut time: &str = "";
    let mut count = 0;

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("12:50");

    loop
    {

        if count == 4{
            count = 0;
            minutes = 25;
            seconds = 0;  

        
            loop{
                clear_screen();
                thread::sleep(time::Duration::from_secs(1));
                if seconds == 0
                {
                    if minutes == 0
                    {
                        is_break = false;
                        break;
                    }
                    else
                    {
                        minutes -= 1;
                        seconds = 59;
                    }
                }
                else
                {
                    seconds -= 1;
                    
                }

                    
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure 
                    {
                        println!("{}", figure);
                    }
                
                

        
            };
            
        }
        if is_break{
            //set 5 min break
            minutes = 5;
            seconds = 0;  

        }
            loop{
                clear_screen();
                thread::sleep(time::Duration::from_secs(1));
                if seconds == 0
                {
                    if minutes == 0
                    {
                        is_break = false;
                        break;
                    }
                    else
                    {
                        minutes -= 1;
                        seconds = 59;
                    }
                }
                else
                {
                    seconds -= 1;
                    
                }

                    
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure 
                    {
                        println!("{}", figure);
                    }
                
                

        
            };
        
        if !is_break{
            //set 20 work time
            minutes = 20;
            seconds = 0;  

        
            loop{
                thread::sleep(time::Duration::from_secs(1));
                clear_screen();
                if seconds == 0
                {
                    if minutes == 0
                    {
                        is_break = true;
                        count += 1;
                        break;
                    }
                    else
                    {
                        minutes -= 1;
                        seconds = 59;
                    }
                }
                else
                {
                    seconds -= 1;
                    
                }

                    
                let time = format!("{}:{:02}", minutes, seconds);
                let figure = standard_font.convert(&time);
                if let Some(ref figure) = figure 
                    {
                        println!("{}", figure);
                    }
                
                

        
            };
        };
    }
}



