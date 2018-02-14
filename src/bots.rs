use rand::{thread_rng, Rng};

pub fn get_bots() -> String {
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

  thread_rng().shuffle(&mut static_bots);

  static_bots[0 .. 3].join(" ")
}
