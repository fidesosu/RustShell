pub fn whereis() {
  //find the current directory
  let path = std::env::current_dir().unwrap();
  println!("        Current directory: {}", path.display());
}