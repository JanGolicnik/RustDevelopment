use duration_string::DurationString;
use notify_rust::Notification;
use std::{env, thread, time::Duration};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1].to_lowercase() == "help" {
            println!("FORMAT: reminders [title] [body] [duration] [icon]");
        }
    } else if args.len() == 5 {
        let title = &args[1];
        let summary = &args[2];
        let duration: Duration = args[3].parse::<DurationString>().unwrap().into();
        let icon = &args[4];
        run(title, summary, &duration, icon);
    } else {
        println!("ERROR: incorrect number of arguments, use > reminders help for usage");
    }
}

fn run(title: &String, summary: &String, duration: &Duration, icon: &String) {
    loop {
        Notification::new()
            .summary(title)
            .body(summary)
            .icon(icon)
            .show()
            .unwrap_or(());
        thread::sleep(*duration);
    }
}
