#[path = "../misc/common/common.rs"]
mod common;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("CC: Usage: cc <C source file> <assembly output file>");
    std::process::exit(1);
  }

  let c_source_file = &args[1];
  let assembly_output_file = &args[2];

  todo!();

  println!("CC: Done");
}
