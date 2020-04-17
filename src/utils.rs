use std::path::PathBuf;
use std::env;

pub fn path_to_string(path: PathBuf) -> String {
  path.into_os_string()
    .into_string()
    .unwrap()
}

pub fn read_env_var(env_var: &str) -> String {
  return match env::var(env_var) {
    Ok(r) => r,
    Err(e) => {
      eprintln!("Couldn't read {} ({})", env_var, e);
      ::std::process::exit(1);
    }
  };
}
