mod fpm;
use fpm::Fpm;

mod project_config;
mod global_config;
mod constants;

fn main() {
  let mut fpm = Fpm::new();
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: fpm <command> [options]");
    return;
  }

  match args[1].as_str() {
    "new" => fpm.create_new_project(&args[2..]),
    "config" => fpm.configure_project(&args[2..]),
    "open" => fpm.open_project(),
    _ => println!("Invalid command"),
  }
}
