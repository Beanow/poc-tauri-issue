#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use tauri::api::{path, shell};
use tauri::Manager;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![set_workaround, shell_open])
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn set_workaround(active: bool) -> bool {
  println!("workaround: {:?}", active);

  let filepath = path::runtime_dir()
    .expect("XDG_RUNTIME_DIR wasn't found")
    .join("flatpak-info");

  if filepath.exists() == active {
    println!("Nothing to do...");
    return active;
  }

  if active {
    fs::File::create(filepath).expect("Couldn't create file");
  } else {
    fs::remove_file(filepath).expect("Couldn't remove file");
  }

  active
}

#[tauri::command]
fn shell_open(window: tauri::Window) {
  let url = "https://github.com/probablykasper/kadium";
  shell::open(&window.shell_scope(), url.to_string(), None).unwrap();
}
