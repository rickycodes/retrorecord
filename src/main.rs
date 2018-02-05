extern crate egg_mode;
extern crate tokio_core;

mod watch;
mod tweet;
mod message;

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    // let recordings = "/home/pi/recordings/";

    loop {
        // println!("{:?}", watch::watch(screenshots));
        // println!("{:?}", watch::watch(recordings))

        let screenshots_path = watch::watch(screenshots);
        if let Ok(_) = tweet::tweet(message::get_message(), screenshots_path) {
            println!("posted tweet!");
        }
    }
}
