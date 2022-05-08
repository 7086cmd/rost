mod rust_compiler {
  pub fn compile(src: String) -> std::process::Output {
    std::process::Command::new("rustc")
      .arg(&src[..])
      .output()
      .expect("Compile Error!")
  }
}

fn main() {
  let result = rust_compiler::compile(String::from("e:/rost/packages/app/src/main.rs"));
}
