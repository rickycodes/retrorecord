mod watch;

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    let recordings = "/home/pi/recordings/";
    loop {
        println!("{:?}", watch::watch(screenshots));
        println!("{:?}", watch::watch(recordings))
    }
}
