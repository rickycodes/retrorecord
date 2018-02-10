extern crate rand;
use self::rand::Rng;

pub fn get_message(bots: String) -> String {
    let messages: [&str; 8] = [
        "I am doing an video game",
        "↑↑↓↓←→←→BA",
        "beep",
        "boop",
        "Hadōken",
        "PRINCESS IS IN ANOTHER CASTLE!",
        "PLAYER1 READY",
        "ONE PLAYER ONLY
            OR
        TWO PLAYERS"
    ];

    let num: usize = rand::thread_rng().gen_range(0, messages.len());
    let message = messages[num].to_string();

    format!("{}

#bot2bot #botALLY

/cc {}", message, bots)
}
