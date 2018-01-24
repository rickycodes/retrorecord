mod watch;

fn main() {
    if let Err(e) = watch::watch() {
        println!("error: {:?}", e)
    }
}
