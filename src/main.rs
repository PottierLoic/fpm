use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: fpm <command> [options]");
    return;
  }

  match args[1].as_str() {
    "new" => create_new_project(&args[2..]),
    "config" => configure_project(&args[2..]),
    "open" => open_project(),
    _ => println!("Invalid command"),
  }
}

fn create_new_project(args: &[String]) {
  if args.len() < 1 {
    println!("Usage: fpm new <project-name>");
    return;
  }

  let project_name = &args[0];
  println!("Creating new project: {}", project_name);
}

fn configure_project(args: &[String]) {
  if args.len() < 1 {
    println!("Usage: fpm config --param1=value --param2=value ...");
    return;
  }

  let param = &args[0];
  println!("Configuring project: {}", param);
}

fn open_project() {
  println!("Opening project");
}
