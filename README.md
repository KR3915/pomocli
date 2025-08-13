# Pomocli
pomocli is simple pomodoro based in terminal with notification feature with simple jsonc config

## Showcase

Insert gif or link to demo
### What is pomodoro
The Pomodoro Technique is a simple way to manage your time. You work for 25 minutes, then take a 5-minute break. After doing that four times, you take a longer break, like 15 to 30 minutes. It helps you stay focused and get things done without burning out.
## How to use
just run the `pomocli` command in your terminal and it will start the timer with the config
### Why i made this project
i needed pomodoro that can run in terminal and i can just adjust the time and messages in config instead of typing the time everytime i want to start the pomodoro and i am also learning rust so i made it myself


## Instalation
clone the repo then build it
```bash
git clone https://github.com/KR3915/pomodoro-cli.git
cd pomodoro-cli
cargo build --release
./target/release/pomodoro-cli
```
## Customization
in ~/.config/pomocli/config.jsonc you can adjust the time for break, long break and work time as well as the messages
```bash
{
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
```


