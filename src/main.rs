#![warn(clippy::pedantic)]

macro_rules! print_with_time {
    ($($arg:tt)*) => {{
        let now = std::time::SystemTime::now();
        let since_the_epoch = now
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");
        let seconds = since_the_epoch.as_secs();
        let hours = (seconds / 3600) % 24;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        let formatted_time = format!("[{:02}:{:02}:{:02}] ", hours, minutes, seconds);
        println!("{formatted_time}{}", format!($($arg)*));
    }};
}

fn main() {
    print_with_time!("Hello, world!");
}
