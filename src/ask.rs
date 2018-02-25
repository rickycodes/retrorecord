use dialoguer::Confirmation;

pub fn ask(question: &str) -> bool {
  let result;
  if Confirmation::new(question).interact().unwrap() {
    result = true;
  } else {
    result = false;
  }

  result
}
