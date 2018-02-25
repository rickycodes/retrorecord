use regex::Regex;

pub fn test_path(path: String, raw: &'static str) -> bool {
  let re = Regex::new(raw).unwrap();
  re.is_match(&path)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_path_matches() {
    assert!(
      test_path("/home/pi/lol/wat/test.png".to_string(),
      r"\.png")
    );
  }

  #[test]
  fn test_path_does_not_match() {
    assert!(
      test_path("/home/pi/lol/wat/test.jpg".to_string(),
      r"\.png"
    ) == false);
  }
}
