use rand::{thread_rng, Rng};

pub fn get_bots(how_many: usize) -> String {
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

  static_bots[0 .. how_many].join(" ")
}

#[cfg(test)]
mod t {
  use super::*;

  #[test]
  fn get_bots_is_correct_length() {
    assert!(get_bots(4).split(" ").count() == 4);
  }
}
