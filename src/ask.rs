use dialoguer::Confirmation;

pub fn ask(question: &str, prompt: bool) -> bool {
  let result;

  if !prompt {
    return true
  }

  if Confirmation::new(question).interact().unwrap() {
    result = true;
  } else {
    result = false;
  }

  result
}
