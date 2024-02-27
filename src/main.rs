mod fpm;
mod project_config;
mod fpm_gui;
mod global_config;
mod constants;

use fpm::Fpm;
use fpm_gui::FpmGui;

fn handle_error(message: &str) {
  println!("{}", message);
  std::process::exit(1);
}

fn main() {
  let mut fpm = Fpm::new().expect("Failed to initialize FPM");
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    let fpm_gui = FpmGui::new();
    fpm_gui.run().unwrap_or_else(|e| handle_error(&format!("Error opening fpm gui: {}", e)));
    println!("Usage: fpm <command> [options]");
    return;
  }

  match args[1].as_str() {
    "new" => {
      fpm.create_new_project(&args[2..])
        .unwrap_or_else(|e| handle_error(&format!("Error creating new project: {}", e)));
    },
    "config" => {
      fpm.configure_project(&args[2..])
        .unwrap_or_else(|e| handle_error(&format!("Error configuring project: {}", e)));
    },
    "open" => {
      fpm.open_project()
        .unwrap_or_else(|e| handle_error(&format!("Error opening project: {}", e)));
    }
    "select" => {
      fpm.select_project(&args[2])
        .unwrap_or_else(|e| handle_error(&format!("Error selecting project: {}", e)));
    }
    _ => {
      handle_error("Invalid command.\nUse 'fpm help' for a list of available commands or consult the man page for more details.");
    },
  }
}
