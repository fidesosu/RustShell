use std::io::{self, Write};
use std::fs::{self};

pub fn remove() -> io::Result<()> {
  // Prompt the user for a file path to delete
  print!("Enter a folder path to delete: ");
  io::stdout().flush()?;
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let folder_path = input.trim();

  // Remove the specified folder and its contents recursively
  let path = std::path::Path::new(folder_path);
  remove_folder_recursive(path)?;

  Ok(())
}

fn remove_file_or_dir(path: &std::path::Path) -> io::Result<()> {
  if path.is_file() {
    if let Err(err) = fs::remove_file(path) {
      println!("Skipping file: {}. Reason: {}", path.display(), err);
    }
  } else if path.is_dir() {
    if let Err(err) = fs::remove_dir_all(path) {
      println!("Skipping folder: {}. Reason: {}", path.display(), err);
    }
  }
  Ok(())
}

fn remove_folder_recursive(path: &std::path::Path) -> io::Result<()> {
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();

    remove_file_or_dir(&path)?;

    if path.is_dir() {
      remove_folder_recursive(&path)?;
    }
  }

  Ok(())
}