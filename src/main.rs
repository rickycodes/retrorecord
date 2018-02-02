extern crate egg_mode;
extern crate tokio_core;

mod watch;
mod tweet;

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    let recordings = "/home/pi/recordings/";
    if let Ok(_) = tweet::tweet("test!") {
        println!("posted tweet!");
    }
    loop {
        println!("{:?}", watch::watch(screenshots));
        println!("{:?}", watch::watch(recordings))
    }
}
