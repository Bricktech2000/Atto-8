#[path = "../misc/common/common.rs"]
mod common;
use common::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Dasm: Usage: dasm <memory image file> <disassembly output file>");
    std::process::exit(1);
  }

  let memory_image_file: &String = &args[1];
  let disassembly_output_file: &String = &args[2];

  let memory_image: [u8; common::MEM_SIZE] = std::fs::read(memory_image_file)
    .unwrap_or_else(|_| {
      println!("Dasm: Error: Unable to read file `{}`", memory_image_file);
      std::process::exit(1);
    })
    .try_into()
    .unwrap_or_else(|_| {
      println!(
        "Dasm: Error: Memory image `{}` has incorrect size",
        memory_image_file,
      );
      std::process::exit(1);
    });

  let opcodes: Vec<u8> = Vec::from(memory_image);

  let instructions: Vec<Result<Instruction, u8>> = opcodes
    .into_iter()
    .map(common::opcode_to_instruction)
    .collect();

  let tokens: Vec<Token> = instructions
    .into_iter()
    .map(common::instruction_to_token)
    .collect();

  let mnemonics: Vec<Mnemonic> = tokens.into_iter().map(common::token_to_mnemonic).collect();

  let disassembly: String = mnemonics
    .into_iter()
    .enumerate()
    .zip(memory_image.into_iter())
    .map(|((index, mnemonic), opcode)| {
      format!(
        "  {} {} # {} {} {}",
        mnemonic,
        Token::AtDyn,
        Token::XXX(index as u8),
        Token::AtOrg,
        Token::AtDD(opcode)
      )
    })
    .collect::<Vec<String>>()
    .join("\n");

  let disassembly = format!(
    "{}\n{}",
    Token::MacroDef(Macro("main".to_string())),
    disassembly
  );

  let disassembly = format!("# Generated by Dasm\n\n{}", disassembly);

  std::fs::write(disassembly_output_file, disassembly).unwrap();

  println!("Dasm: Done");
}
