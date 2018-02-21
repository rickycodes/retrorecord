use dialoguer::Confirmation;

pub fn ask(question: &'static str) -> bool {
  let result;
  if Confirmation::new(&question).interact().unwrap() {
    println!("yo sick!");
    result = true;
  } else {
    println!("boo 💩");
    result = false;
  }

  result
}
