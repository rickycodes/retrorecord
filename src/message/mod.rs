extern crate rand;
use self::rand::{thread_rng, Rng};

pub fn get_message() -> String {
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

    let mut static_bots: [&str; 12] = [
        "@ArtyMash",
        "@ArtyAbstract",
        "@ArtyCurve",
        "@ArtyNegative",
        "@ArtyWinds",
        "@pixelsorter",
        "@ImgShuffleBOT",
        "@DeepDreamThis",
        "@a_quilt_bot",
        "@IMG2ASCII",
        "@kaleid_o_bot",
        "@picwhip"
    ];

    let num: usize = rand::thread_rng().gen_range(0, messages.len());
    let message = messages[num].to_string();

    thread_rng().shuffle(&mut static_bots);

    format!("{}

#bot2bot #botALLY

/cc {}", message, static_bots[0 .. 3].join(" "))
}
