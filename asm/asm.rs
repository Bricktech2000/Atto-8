use std::collections::{HashMap, HashSet};

#[path = "../misc/common/common.rs"]
mod common;
use common::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
    println!("Asm: Usage: asm <assembly source file> <memory image file>");
    std::process::exit(1);
  }

  let mut errors: Vec<(Pos, Error)> = vec![];
  let memory_image_file = &args[2];
  let assembly_source_file: File = File(args[1].clone());

  let preprocessed: String = preprocess(assembly_source_file, &mut errors, None);
  let mnemonics: Vec<(Pos, Mnemonic)> = mnemonize(preprocessed, &mut errors);
  let tokens: Vec<(Pos, Token)> = tokenize(mnemonics, &mut errors);
  let instructions: Vec<(Pos, Result<Instruction, u8>)> = assemble(tokens, &mut errors, "main");
  let opcodes: Vec<(Pos, u8)> = codegen(instructions, &mut errors);
  let memory_image: Vec<(Pos, u8)> = opcodes;

  match errors[..] {
    [] => {
      std::fs::write::<&String, [u8; common::MEM_SIZE]>(
        memory_image_file,
        memory_image
          .iter()
          .map(|(_, b)| *b)
          .collect::<Vec<u8>>()
          .try_into()
          .unwrap(),
      )
      .unwrap();
    }
    _ => {
      let errors = errors
        .iter()
        .map(|(pos, error)| format!("Asm: Error: {}: {}", pos, error))
        .collect::<Vec<String>>()
        .join("\n");

      println!("{}", errors);
      std::process::exit(1);
    }
  }

  println!("Asm: Done");
}

#[derive(Clone, Eq, PartialEq)]
enum Root {
  Instruction(Instruction),
  LabelDef(Label),
  Node(Node),
  Opcode(u8),
  Const,
  Dyn(Option<Instruction>),
  Org(Option<Node>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Node {
  LabelRef(Label),
  Value(u8),
  Add(Box<Node>, Box<Node>),
  Sub(Box<Node>, Box<Node>),
  Rot(Box<Node>, Box<Node>),
  Orr(Box<Node>, Box<Node>),
  And(Box<Node>, Box<Node>),
  Xor(Box<Node>, Box<Node>),
  Xnd(Box<Node>, Box<Node>),
  Shl(Box<Node>),
  Shr(Box<Node>),
  Not(Box<Node>),
}

fn preprocess(file: File, errors: &mut Vec<(Pos, Error)>, scope: Option<&str>) -> String {
  // remove comments and resolve includes

  use std::path::Path;
  let assembly = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    errors.push((
      Pos(scope.unwrap_or("[bootstrap]").to_string(), 0),
      Error(format!("Unable to read file `{}`", file)),
    ));
    format!("")
  });

  let assembly: String = assembly
    .lines()
    .map(|line| line.strip_suffix("#").unwrap_or(line))
    .map(|line| line.split("# ").next().unwrap_or(line))
    .map(|line| match line.find("@ ") {
      Some(i) => {
        line[..i].to_owned()
          + preprocess(
            File(
              Path::new(&file.0)
                .parent()
                .unwrap()
                .join(&line[i..]["@ ".len()..])
                .to_str()
                .unwrap()
                .to_string(),
            ),
            errors,
            Some(&format!("{}", file)),
          )
          .as_str()
      }
      None => line.to_string(),
    })
    .map(|line| line.to_string() + "\n")
    .collect::<String>();

  assembly
}

fn mnemonize(assembly: String, _errors: &mut Vec<(Pos, Error)>) -> Vec<(Pos, Mnemonic)> {
  let mnemonics: Vec<(Pos, Mnemonic)> = assembly
    .split_whitespace()
    .map(|mnemonic| Mnemonic(mnemonic.to_string()))
    .enumerate()
    .map(|(index, mnemonic)| (Pos("[token stream]".to_string(), index), mnemonic))
    .collect();

  mnemonics
}

fn tokenize(mnemonics: Vec<(Pos, Mnemonic)>, errors: &mut Vec<(Pos, Error)>) -> Vec<(Pos, Token)> {
  // tokenize to valid tokens. tokens might be invalid instructions

  let tokens: Vec<(Pos, Token)> = mnemonics
    .into_iter()
    .map(|(pos, mnemonic)| {
      (
        pos.clone(),
        match common::mnemonic_to_token(mnemonic.clone()) {
          Some(token) => token,
          None => {
            errors.push((
              pos.clone(),
              Error(format!("Invalid mnemonic `{}`", mnemonic)),
            ));
            Token::Nop
          }
        },
      )
    })
    .collect();

  tokens
}

fn assemble(
  tokens: Vec<(Pos, Token)>,
  errors: &mut Vec<(Pos, Error)>,
  entry_point: &str,
) -> Vec<(Pos, Result<Instruction, u8>)> {
  // resolve macros recursively from `entry_point` and identify unused labels

  let mut macro_definitions: HashMap<Macro, Vec<(Pos, Token)>> = HashMap::new();
  let mut current_macro: Option<Macro> = None;

  for (pos, token) in tokens.into_iter() {
    match token {
      Token::MacroDef(macro_) => {
        current_macro = Some(macro_.clone());
        macro_definitions
          .entry(macro_.clone())
          .and_modify(|_| {
            errors.push((
              pos.clone(),
              Error(format!("Duplicate macro definition `{}`", macro_)),
            ));
          })
          .or_insert(vec![]);
      }

      _ => match current_macro
        .as_ref()
        .and_then(|macro_| macro_definitions.get_mut(&macro_))
      {
        Some(macro_tokens) => macro_tokens.push((
          Pos(
            format!("{}", current_macro.as_ref().unwrap()),
            macro_tokens.len(),
          ),
          token,
        )),
        None => errors.push((pos, Error(format!("Orphan token `{}` encountered", token)))),
      },
    }
  }

  let tokens = expand_macros(
    &vec![(
      Pos("[bootstrap]".to_string(), 0),
      Token::MacroRef(Macro(entry_point.to_string())),
    )],
    &mut 0,
    &mut vec![],
    &macro_definitions,
    errors,
  );

  fn expand_macros(
    tokens: &Vec<(Pos, Token)>,
    scope_uid: &mut usize,
    parent_macros: &mut Vec<Macro>,
    macro_definitions: &HashMap<Macro, Vec<(Pos, Token)>>,
    errors: &mut Vec<(Pos, Error)>,
  ) -> Vec<(Pos, Token)> {
    tokens
      .into_iter()
      .flat_map(|(pos, token)| match token {
        Token::MacroRef(macro_) => {
          if parent_macros.contains(&macro_) {
            errors.push((
              pos.clone(),
              Error(format!(
                "Macro self-reference {} -> `{}`",
                parent_macros
                  .iter()
                  .map(|macro_| format!("`{}`", macro_))
                  .collect::<Vec<String>>()
                  .join(" -> "),
                macro_
              )),
            ));
            return vec![];
          }

          let tokens = match macro_definitions.get(&macro_) {
            Some(tokens) => tokens.clone(),
            None => {
              errors.push((pos.clone(), Error(format!("Undefined macro `{}`", macro_))));
              vec![]
            }
          };

          let tokens = tokens
            .into_iter()
            .map(|(pos, token)| match token {
              Token::LabelDef(Label::Local(identifier, _)) => (
                pos,
                Token::LabelDef(Label::Local(identifier, Some(*scope_uid))),
              ),
              Token::LabelRef(Label::Local(identifier, _)) => (
                pos,
                Token::LabelRef(Label::Local(identifier, Some(*scope_uid))),
              ),
              _ => (pos, token),
            })
            .collect();

          *scope_uid += 1;
          parent_macros.push(macro_.clone());
          let tokens = expand_macros(
            &tokens,
            scope_uid,
            parent_macros,
            &macro_definitions,
            errors,
          );
          parent_macros.pop();

          tokens
        }

        Token::AtErr => {
          errors.push((
            pos.clone(),
            Error(format!("`{}` directive encountered", token)),
          ));
          vec![]
        }
        _ => vec![(pos.clone(), token.clone())],
      })
      .collect()
  }

  let label_definitions: HashMap<Label, Pos> = tokens
    .iter()
    .filter_map(|(pos, token)| match token {
      Token::LabelDef(label) => Some((label.clone(), pos.clone())),
      _ => None,
    })
    .collect();

  let label_references: HashSet<Label> = tokens
    .iter()
    .filter_map(|(_pos, token)| match token {
      Token::LabelRef(label) => Some(label.clone()),
      _ => None,
    })
    .collect();

  errors.extend(label_definitions.iter().filter_map(|(label, pos)| {
    (!label_references.contains(label))
      .then_some((pos.clone(), Error(format!("Unused label `{}`", label))))
  }));

  // turn assembly tokens into roots, an intermediate representation for optimization. roots correspond to valid instructions

  #[allow(dead_code)]
  fn assert_imm(imm: u8, errors: &mut Vec<(Pos, Error)>, pos: &Pos) -> u8 {
    match imm {
      0b00000000..=0b01111111 => imm,
      _ => {
        errors.push((
          pos.clone(),
          Error(format!("Invalid IMM operand `{:02X}`", imm)),
        ));
        0b00000000
      }
    }
  }

  fn assert_size(size: u8, errors: &mut Vec<(Pos, Error)>, pos: &Pos) -> u8 {
    match size {
      0x01 | 0x02 | 0x04 | 0x08 => size,
      _ => {
        errors.push((
          pos.clone(),
          Error(format!("Invalid SIZE operand `{:02X}`", size)),
        ));
        0x01
      }
    }
  }

  fn assert_ofst(ofst: u8, errors: &mut Vec<(Pos, Error)>, pos: &Pos) -> u8 {
    match ofst {
      0b00000000..=0b00001111 => ofst,
      _ => {
        errors.push((
          pos.clone(),
          Error(format!("Invalid OFST operand `{:02X}`", ofst)),
        ));
        0b00000000
      }
    }
  }

  #[allow(dead_code)]
  fn assert_nimm(nimm: u8, errors: &mut Vec<(Pos, Error)>, pos: &Pos) -> u8 {
    match nimm {
      0b11110000..=0b11111111 => nimm,
      _ => {
        errors.push((
          pos.clone(),
          Error(format!("Invalid NIMM operand `{:02X}`", nimm)),
        ));
        0b00000000
      }
    }
  }

  let roots: Vec<(Pos, Root)> = tokens
    .into_iter()
    .map(|(pos, token)| {
      let token = match token {
        Token::LabelDef(label) => Root::LabelDef(label),
        Token::LabelRef(label) => Root::Node(Node::LabelRef(label)),
        Token::MacroDef(_) => panic!("Macro definition found in intermediate representation"),
        Token::MacroRef(_) => panic!("Macro reference found in intermediate representation"),
        Token::AtConst => Root::Const,
        Token::AtDyn => Root::Dyn(None),
        Token::AtOrg => Root::Org(None),
        Token::AtErr => panic!("Error directive found in intermediate representation"),
        Token::XXX(value) => Root::Node(Node::Value(value)),
        Token::Add => Root::Instruction(Instruction::Add(assert_size(0x01, errors, &pos))),
        Token::AdS(size) => Root::Instruction(Instruction::Add(assert_size(size, errors, &pos))),
        Token::Sub => Root::Instruction(Instruction::Sub(assert_size(0x01, errors, &pos))),
        Token::SuS(size) => Root::Instruction(Instruction::Sub(assert_size(size, errors, &pos))),
        Token::Iff => Root::Instruction(Instruction::Iff(assert_size(0x01, errors, &pos))),
        Token::IfS(size) => Root::Instruction(Instruction::Iff(assert_size(size, errors, &pos))),
        Token::Rot => Root::Instruction(Instruction::Rot(assert_size(0x01, errors, &pos))),
        Token::RoS(size) => Root::Instruction(Instruction::Rot(assert_size(size, errors, &pos))),
        Token::Orr => Root::Instruction(Instruction::Orr(assert_size(0x01, errors, &pos))),
        Token::OrS(size) => Root::Instruction(Instruction::Orr(assert_size(size, errors, &pos))),
        Token::And => Root::Instruction(Instruction::And(assert_size(0x01, errors, &pos))),
        Token::AnS(size) => Root::Instruction(Instruction::And(assert_size(size, errors, &pos))),
        Token::Xor => Root::Instruction(Instruction::Xor(assert_size(0x01, errors, &pos))),
        Token::XoS(size) => Root::Instruction(Instruction::Xor(assert_size(size, errors, &pos))),
        Token::Xnd => Root::Instruction(Instruction::Xnd(assert_size(0x01, errors, &pos))),
        Token::XnS(size) => Root::Instruction(Instruction::Xnd(assert_size(size, errors, &pos))),
        Token::Inc => Root::Instruction(Instruction::Inc),
        Token::Dec => Root::Instruction(Instruction::Dec),
        Token::Neg => Root::Instruction(Instruction::Neg),
        Token::Shl => Root::Instruction(Instruction::Shl),
        Token::Shr => Root::Instruction(Instruction::Shr),
        Token::Not => Root::Instruction(Instruction::Not),
        Token::Buf => Root::Instruction(Instruction::Buf),
        Token::LdO(ofst) => Root::Instruction(Instruction::Ldo(assert_ofst(ofst, errors, &pos))),
        Token::StO(ofst) => Root::Instruction(Instruction::Sto(assert_ofst(ofst, errors, &pos))),
        Token::Lda => Root::Instruction(Instruction::Lda),
        Token::Sta => Root::Instruction(Instruction::Sta),
        Token::Ldi => Root::Instruction(Instruction::Ldi),
        Token::Sti => Root::Instruction(Instruction::Sti),
        Token::Lds => Root::Instruction(Instruction::Lds),
        Token::Sts => Root::Instruction(Instruction::Sts),
        Token::Nop => Root::Instruction(Instruction::Nop),
        Token::Clc => Root::Instruction(Instruction::Clc),
        Token::Sec => Root::Instruction(Instruction::Sec),
        Token::Flc => Root::Instruction(Instruction::Flc),
        Token::Swp => Root::Instruction(Instruction::Swp),
        Token::Pop => Root::Instruction(Instruction::Pop),
        Token::AtDD(0xBB) => Root::Instruction(Instruction::Dbg),
        Token::AtDD(value) => Root::Opcode(value),
      };

      (pos, token)
    })
    .collect();

  let roots = optimize(roots, errors);

  // assemble roots into instructions by computing the value of every node and resolving labels

  fn build_push_instruction(value: u8, pos: &Pos) -> Vec<(Pos, Instruction)> {
    // the `Psh` instruction allows us to push arbitrary 7-bit immediates onto the stack.
    // we then optionally use `Neg` and `Inc` to get the ability to push arbitrary 8-bit
    // values. we also use `Phn` as a shorthand when possible.

    match value {
      0b11110000..=0b11111111 => vec![(pos.clone(), Instruction::Phn(value))],
      0b10000000..=0b10000000 => vec![
        (pos.clone(), Instruction::Psh(value.wrapping_sub(1))),
        (pos.clone(), Instruction::Inc),
      ],
      0b00000000..=0b01111111 => vec![(pos.clone(), Instruction::Psh(value))],
      0b10000000..=0b11111111 => vec![
        (pos.clone(), Instruction::Psh(value.wrapping_neg())),
        (pos.clone(), Instruction::Neg),
      ],
    }
  }

  // if every label a node depends on could be resolved, we can replace it with a value.
  // if not, start by allocating one byte for pushing the node later. if pushing the node turns
  // out to require more than one byte, iteratively `'bruteforce` allocation sizes until we
  // find one that works. repeat for every node.

  let mut instructions: Vec<(Pos, Result<Instruction, u8>)>;
  let mut allocation_sizes: HashMap<Node, usize> = HashMap::new();

  'bruteforce: loop {
    let mut location_counter: usize = 0;
    let mut label_definitions: HashMap<Label, u8> = HashMap::new();
    let mut unevaluated_nodes: HashMap<u8, (Pos, Node)> = HashMap::new();

    instructions = roots
      .clone()
      .into_iter()
      .flat_map(|(pos, root)| match root {
        Root::Instruction(instruction) | Root::Dyn(Some(instruction)) => {
          let instructions_ = vec![(pos, Ok(instruction))];
          location_counter += instructions_.len();
          instructions_
        }

        Root::Node(node) => match resolve_node_value(&node, &label_definitions) {
          Ok(value) => {
            let instructions_ = build_push_instruction(value, &pos)
              .into_iter()
              .map(|(pos, instruction)| (pos, Ok(instruction)))
              .collect::<Vec<_>>();
            location_counter += instructions_.len();
            instructions_
          }
          Err(_) => {
            let instructions_ = vec![
              (pos.clone(), Ok(Instruction::Nop));
              allocation_sizes.get(&node).copied().unwrap_or(1)
            ];
            unevaluated_nodes.insert(location_counter as u8, (pos, node));
            location_counter += instructions_.len();
            instructions_
          }
        },

        Root::Opcode(opcode) => {
          let instructions_ = vec![(pos, Err(opcode))];
          location_counter += instructions_.len();
          instructions_
        }

        Root::LabelDef(Label::Local(_, None)) => panic!("Local label has no scope specified"),

        Root::LabelDef(label) => {
          if label_definitions.contains_key(&label) {
            errors.push((
              pos,
              Error(format!("Duplicate label definition `{}`", label)),
            ));
          }
          label_definitions.insert(label, location_counter as u8);
          vec![]
        }

        Root::Org(Some(node)) => match resolve_node_value(&node, &label_definitions) {
          Ok(value) => {
            if value as usize >= location_counter {
              let difference = value as usize - location_counter;
              location_counter += difference;
              vec![(pos, Err(0x00)); difference as usize]
            } else {
              errors.push((
                pos,
                Error(format!(
                  "`{}` cannot move location counter backward from `{:02X}` to `{:02X}`",
                  Token::AtOrg,
                  location_counter,
                  value
                )),
              ));
              vec![]
            }
          }
          Err(label) => {
            errors.push((
              pos,
              Error(format!(
                "`{}` argument contains currently unresolved label `{}`",
                Token::AtOrg,
                label
              )),
            ));
            vec![]
          }
        },

        Root::Org(None) => {
          errors.push((
            pos,
            Error(format!(
              "`{}` argument could not be reduced to a constant expression",
              Token::AtOrg,
            )),
          ));
          vec![]
        }

        Root::Const => {
          errors.push((
            pos,
            Error(format!(
              "`{}` argument could not be reduced to a constant expression",
              Token::AtConst,
            )),
          ));
          vec![]
        }

        Root::Dyn(None) => {
          errors.push((
            pos,
            Error(format!(
              "`{}` argument could not be reduced to an instruction",
              Token::AtDyn,
            )),
          ));
          vec![]
        }
      })
      .collect();

    // poke into `instructions` and evaluate the nodes that couldn't be evaluated before
    'poke: {
      for (location_counter, (pos, node)) in unevaluated_nodes.iter() {
        let value = match resolve_node_value(&node, &label_definitions) {
          Ok(value) => value,
          Err(label) => {
            errors.push((pos.clone(), Error(format!("Undefined label `{}`", label))));
            0x00
          }
        };

        // if the evaluated node doesn't fit in the allocated memory, note down the right amount of
        // memory to allocate on the next iteration of `'bruteforce` and try again

        let instructions_ = build_push_instruction(value, &pos);
        if instructions_.len() > allocation_sizes.get(&node).copied().unwrap_or(1) {
          allocation_sizes.insert(node.clone(), instructions_.len());
          break 'poke;
        }

        for (index, (pos, instruction)) in instructions_.into_iter().enumerate() {
          instructions[*location_counter as usize + index] = (pos, Ok(instruction));
        }
      }

      // all unevaluated nodes have been evaluated, break out of the bruteforce loop
      break 'bruteforce;
    }

    // abort brute force if errors were encountered
    if errors.len() > 0 {
      break 'bruteforce;
    }
  }

  instructions
}

fn codegen(
  instructions: Vec<(Pos, Result<Instruction, u8>)>,
  errors: &mut Vec<(Pos, Error)>,
) -> Vec<(Pos, u8)> {
  // codegen instructions into opcodes

  let opcodes: Vec<(Pos, u8)> = instructions
    .into_iter()
    .map(|(pos, instruction)| (pos, common::instruction_to_opcode(instruction)))
    .collect();

  let mut opcodes = opcodes;
  let pos = Pos("[codegen]".to_string(), 0);

  match common::MEM_SIZE.checked_sub(opcodes.len()) {
    Some(padding) => opcodes.extend(vec![(pos, 0x00); padding]),
    None => {
      errors.push((
        pos,
        Error(format!(
          "Program size `{:02X}` exceeds available memory of size `{:02X}`",
          opcodes.len(),
          common::MEM_SIZE
        )),
      ));
    }
  }

  opcodes
}

fn optimize(roots: Vec<(Pos, Root)>, _errors: &mut Vec<(Pos, Error)>) -> Vec<(Pos, Root)> {
  // build a tree of nodes representing everything we can compute at compile time
  // this removes redundant instructions and makes macros usable

  // a convenience function to replace slice patterns within a vector
  fn match_replace<const N: usize>(
    roots: &Vec<(Pos, Root)>,
    replacer: fn(&[Root; N]) -> Option<Vec<Root>>,
  ) -> Vec<(Pos, Root)> {
    if roots.len() < N {
      return roots.clone();
    }

    let mut output: Vec<(Pos, Root)> = vec![];

    let mut skip_next_n_roots = 0;
    for window in roots.windows(N) {
      if skip_next_n_roots > 0 {
        skip_next_n_roots -= 1;
      } else {
        match replacer(
          window
            .iter()
            .map(|(_, root)| root.clone())
            .collect::<Vec<Root>>()
            .as_slice()
            .try_into()
            .unwrap(),
        ) {
          Some(roots) => {
            output.extend(
              roots
                .into_iter()
                .map(|root| (window[0].0.clone(), root))
                .collect::<Vec<(Pos, Root)>>(),
            );
            skip_next_n_roots = N - 1;
          }
          None => output.push(window[0].clone()),
        }
      }
    }
    output.extend(
      roots
        .iter()
        .skip(1 + roots.len() - N + skip_next_n_roots)
        .cloned(),
    );

    output
  }

  #[derive(Clone, Eq, PartialEq)]
  enum OpType {
    NoOp,     // 0 -> 0
    PushOp,   // 0 -> 1
    PopOp,    // 1 -> 0
    UnaryOp,  // 1 -> 1
    BinaryOp, // 2 -> 1
    DualOp,   // 2 -> 2
    Impure,   // has side effects
  }

  // this function maps roots to the effect they have on the stack. if a root is not to be optimized away
  // because it produces a side effect in the form of a write to memory or to a register, it is mapped to
  // `Impure`. writing to `CF` and reading from memory or from a register are not considered side effects
  fn op_type(root: &Root) -> OpType {
    match root {
      Root::Instruction(instruction) => match instruction {
        Instruction::Psh(_imm) => OpType::PushOp,
        Instruction::Add(0x01) => OpType::BinaryOp,
        Instruction::Add(_size) => OpType::Impure,
        Instruction::Sub(0x01) => OpType::BinaryOp,
        Instruction::Sub(_size) => OpType::Impure,
        Instruction::Iff(0x01) => OpType::BinaryOp,
        Instruction::Iff(_size) => OpType::Impure,
        Instruction::Rot(0x01) => OpType::BinaryOp,
        Instruction::Rot(_size) => OpType::Impure,
        Instruction::Orr(0x01) => OpType::BinaryOp,
        Instruction::Orr(_size) => OpType::Impure,
        Instruction::And(0x01) => OpType::BinaryOp,
        Instruction::And(_size) => OpType::Impure,
        Instruction::Xor(0x01) => OpType::BinaryOp,
        Instruction::Xor(_size) => OpType::Impure,
        Instruction::Xnd(0x01) => OpType::BinaryOp,
        Instruction::Xnd(_size) => OpType::Impure,
        Instruction::Inc => OpType::UnaryOp,
        Instruction::Dec => OpType::UnaryOp,
        Instruction::Neg => OpType::UnaryOp,
        Instruction::Shl => OpType::UnaryOp,
        Instruction::Shr => OpType::UnaryOp,
        Instruction::Not => OpType::UnaryOp,
        Instruction::Buf => OpType::NoOp,
        Instruction::Dbg => OpType::Impure,
        Instruction::Ldo(_ofst) => OpType::PushOp,
        Instruction::Sto(_ofst) => OpType::Impure,
        Instruction::Lda => OpType::UnaryOp,
        Instruction::Sta => OpType::Impure,
        Instruction::Ldi => OpType::PushOp,
        Instruction::Sti => OpType::Impure,
        Instruction::Lds => OpType::PushOp,
        Instruction::Sts => OpType::Impure,
        Instruction::Nop => OpType::NoOp,
        Instruction::Clc => OpType::Impure, // `clc` is to be left unaltered
        Instruction::Sec => OpType::Impure, // `sec` is to be left unaltered
        Instruction::Flc => OpType::Impure, // `flc` is to be left unaltered
        Instruction::Swp => OpType::DualOp,
        Instruction::Pop => OpType::PopOp,
        Instruction::Phn(_nimm) => OpType::PushOp,
      },
      Root::LabelDef(_) => OpType::Impure,
      Root::Node(_) => OpType::PushOp,
      Root::Opcode(_) => OpType::Impure,
      Root::Const => OpType::Impure,
      Root::Dyn(_) => OpType::Impure,
      Root::Org(_) => OpType::Impure,
    }
  }

  let mut roots = roots;

  // optimize as much as possible into `Node`s for assembly-time evaluation

  let mut last_roots = vec![];
  while roots != last_roots {
    last_roots = roots.clone();
    // println!("roots: {:?}\nlen: {}", roots, roots.len());

    // directives
    roots = match_replace(&roots, |window| match window {
      [Root::Node(node), Root::Const] => Some(vec![Root::Node(node.clone())]),

      [Root::Instruction(instruction), Root::Dyn(None)] => {
        Some(vec![Root::Dyn(Some(instruction.clone()))])
      }

      [Root::Dyn(Some(instruction)), Root::Dyn(None)] => {
        Some(vec![Root::Dyn(Some(instruction.clone()))])
      }

      [Root::Opcode(opcode), Root::Dyn(None)] => Some(vec![Root::Opcode(opcode.clone())]),

      [Root::Node(node), Root::Org(None)] => Some(vec![Root::Org(Some(node.clone()))]),

      _ => None,
    });
    roots = match_replace(&roots, |window| match window {
      // for `!pad` macro
      [Root::Node(node), Root::LabelDef(label), Root::Org(None)] => Some(vec![
        Root::LabelDef(label.clone()),
        Root::Node(node.clone()),
        Root::Org(None),
      ]),

      [Root::Node(node), Root::LabelDef(label), Root::Const] => Some(vec![
        Root::Node(node.clone()),
        Root::Const,
        Root::LabelDef(label.clone()),
      ]),

      _ => None,
    });

    // length 1
    roots = match_replace(&roots, |window| match window {
      // `OpType`s
      [no_op] if op_type(no_op) == OpType::NoOp => Some(vec![]),
      _ => None,
    });

    // length 2
    roots =
      match_replace(&roots, |window| match window {
        [Root::Node(x00), Root::Instruction(Instruction::Add(_size))]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }

        [Root::Node(x01), Root::Instruction(Instruction::Add(0x01))]
          if resolve_node_value(&x01, &HashMap::new()) == Ok(0x01) =>
        {
          Some(vec![Root::Instruction(Instruction::Inc)])
        }

        [Root::Node(x00), Root::Instruction(Instruction::Sub(_size))]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }

        [Root::Node(x01), Root::Instruction(Instruction::Sub(0x01))]
          if resolve_node_value(&x01, &HashMap::new()) == Ok(0x01) =>
        {
          Some(vec![Root::Instruction(Instruction::Dec)])
        }

        [Root::Node(div_by_eight), Root::Instruction(Instruction::Rot(_size))]
          if resolve_node_value(&div_by_eight, &HashMap::new()).map(|value| value % 8)
            == Ok(0x00) =>
        {
          Some(vec![])
        }

        [Root::Node(x00), Root::Instruction(Instruction::Orr(_size))]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }

        [Root::Node(ff), Root::Instruction(Instruction::And(_size))]
          if resolve_node_value(&ff, &HashMap::new()) == Ok(0xFF) =>
        {
          Some(vec![])
        }

        [Root::Node(x00), Root::Instruction(Instruction::Xor(_size))]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          Some(vec![])
        }

        [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(
          Node::Add(Box::new(Node::Value(0x01)), Box::new(node.clone())),
        )]),

        [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(
          Node::Sub(Box::new(Node::Value(0x01)), Box::new(node.clone())),
        )]),

        [Root::Node(node), Root::Instruction(Instruction::Neg)] => Some(vec![Root::Node(
          Node::Sub(Box::new(node.clone()), Box::new(Node::Value(0x00))),
        )]),

        [Root::Instruction(Instruction::Neg), Root::Instruction(Instruction::Neg)] => Some(vec![]),

        [Root::Node(node), Root::Instruction(Instruction::Shl)] => {
          Some(vec![Root::Node(Node::Shl(Box::new(node.clone())))])
        }

        [Root::Node(node), Root::Instruction(Instruction::Shr)] => {
          Some(vec![Root::Node(Node::Shr(Box::new(node.clone())))])
        }

        [Root::Node(node), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Node(Node::Not(Box::new(node.clone())))])
        }

        [Root::Instruction(Instruction::Not), Root::Instruction(Instruction::Not)] => {
          Some(vec![Root::Instruction(Instruction::Buf)])
        }

        [Root::Node(node), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Node(node.clone())])
        }

        [Root::Instruction(Instruction::Buf), Root::Instruction(Instruction::Buf)] => {
          Some(vec![Root::Instruction(Instruction::Buf)])
        }

        [Root::Node(node), Root::Instruction(Instruction::Ldo(0x00))] => {
          Some(vec![Root::Node(node.clone()), Root::Node(node.clone())])
        }

        [Root::Instruction(Instruction::Clc), Root::Instruction(Instruction::Clc)] => {
          Some(vec![Root::Instruction(Instruction::Clc)])
        }

        [Root::Instruction(Instruction::Sec), Root::Instruction(Instruction::Sec)] => {
          Some(vec![Root::Instruction(Instruction::Sec)])
        }

        [Root::Instruction(Instruction::Flc), Root::Instruction(Instruction::Flc)] => Some(vec![]),

        [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Swp)] => Some(vec![]),

        [push_op, pop_op]
          if op_type(push_op) == OpType::PushOp && op_type(pop_op) == OpType::PopOp =>
        {
          Some(vec![])
        }

        [unary_op, pop_op]
          if op_type(unary_op) == OpType::UnaryOp && op_type(pop_op) == OpType::PopOp =>
        {
          Some(vec![pop_op.clone()])
        }

        _ => None,
      });

    // length 3
    roots = match_replace(&roots, |window| match window {
      // `Node`s
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Add(0x01))] => {
        Some(vec![Root::Node(Node::Add(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Sub(0x01))] => {
        Some(vec![Root::Node(Node::Sub(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Rot(0x01))] => {
        Some(vec![Root::Node(Node::Rot(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Orr(0x01))] => {
        Some(vec![Root::Node(Node::Orr(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::And(0x01))] => {
        Some(vec![Root::Node(Node::And(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xor(0x01))] => {
        Some(vec![Root::Node(Node::Xor(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xnd(0x01))] => {
        Some(vec![Root::Node(Node::Xnd(
          Box::new(node2.clone()),
          Box::new(node1.clone()),
        ))])
      }

      // `Ldo`s
      [Root::Node(node), push_op, Root::Instruction(Instruction::Ldo(0x01))]
        if op_type(push_op) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(node.clone()),
          push_op.clone(),
          Root::Node(node.clone()),
        ])
      }

      // `Swp`s
      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Inc), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Node(Node::Value(0x01)),
          Root::Instruction(Instruction::Add(0x02)),
        ])
      }
      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Dec), Root::Instruction(Instruction::Swp)] => {
        Some(vec![
          Root::Node(Node::Value(0x01)),
          Root::Instruction(Instruction::Sub(0x02)),
        ])
      }
      [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Swp)] => {
        Some(vec![Root::Node(node2.clone()), Root::Node(node1.clone())])
      }
      [Root::Instruction(Instruction::Ldo(ofst)), Root::Node(node2), Root::Instruction(Instruction::Swp)]
        if *ofst < 0b00001111 =>
      {
        Some(vec![
          Root::Node(node2.clone()),
          Root::Instruction(Instruction::Ldo(*ofst + 1)),
        ])
      }
      [Root::Node(node1), Root::Instruction(Instruction::Ldo(ofst)), Root::Instruction(Instruction::Swp)]
        if *ofst > 0b00000000 =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*ofst - 1)),
          Root::Node(node1.clone()),
        ])
      }
      [Root::Instruction(Instruction::Ldo(ofst1)), Root::Instruction(Instruction::Ldo(ofst2)), Root::Instruction(Instruction::Swp)]
        if *ofst1 < 0b00001111 && *ofst2 > 0b00000000 =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(*ofst2 - 1)),
          Root::Instruction(Instruction::Ldo(*ofst1 + 1)),
        ])
      }

      // `Sto`s
      [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(0x07))]
      | [Root::Node(x00), Root::Instruction(Instruction::Sto(0x08)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        // OpType::PopOp
        Some(vec![Root::Instruction(Instruction::Xnd(0x08))])
      }
      [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(0x03))]
      | [Root::Node(x00), Root::Instruction(Instruction::Sto(0x04)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        // OpType::PopOp
        Some(vec![Root::Instruction(Instruction::Xnd(0x04))])
      }
      [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(0x01))]
      | [Root::Node(x00), Root::Instruction(Instruction::Sto(0x02)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        // OpType::PopOp
        Some(vec![Root::Instruction(Instruction::Xnd(0x02))])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x00)]
      | [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(x00), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        // OpType::PopOp
        Some(vec![Root::Instruction(Instruction::Xnd(0x01))])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x01)]
      | [Root::Instruction(Instruction::Pop), Root::Node(x01), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(x01), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x01, &HashMap::new()) == Ok(0x01) =>
      {
        // OpType::PopOp
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Shl),
        ])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x80)]
      | [Root::Instruction(Instruction::Pop), Root::Node(x80), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(x80), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&x80, &HashMap::new()) == Ok(0x80) =>
      {
        // OpType::PopOp
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Shr),
        ])
      }
      [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(xff)]
      | [Root::Instruction(Instruction::Pop), Root::Node(xff), Root::Instruction(Instruction::Sto(0x00))]
      | [Root::Node(xff), Root::Instruction(Instruction::Sto(0x01)), Root::Instruction(Instruction::Pop)]
        if resolve_node_value(&xff, &HashMap::new()) == Ok(0xFF) =>
      {
        // OpType::PopOp
        Some(vec![
          Root::Instruction(Instruction::Xnd(0x01)),
          Root::Instruction(Instruction::Not),
        ])
      }

      // `OpType`s
      [push_op, binary_op, pop_op]
        if op_type(push_op) == OpType::PushOp
          && op_type(binary_op) == OpType::BinaryOp
          && op_type(pop_op) == OpType::PopOp =>
      {
        Some(vec![pop_op.clone()])
      }
      [dual_op, pop_op1, pop_op2]
        if op_type(dual_op) == OpType::DualOp
          && op_type(pop_op1) == OpType::PopOp
          && op_type(pop_op2) == OpType::PopOp =>
      {
        Some(vec![])
      }

      _ => None,
    });

    // length 4
    roots = match_replace(&roots, |window| {
      match window {
        // doubled `BinaryOp`s
        [Root::Node(node1), Root::Instruction(Instruction::Add(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Add(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Add(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Add(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Sub(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Add(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Sub(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Sub(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Sub(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Sub(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Add(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Sub(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Rot(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Rot(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Rot(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Orr(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Orr(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Orr(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::And(same_size1)), Root::Node(node2), Root::Instruction(Instruction::And(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::And(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Xor(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Xor(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Xor(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Xnd(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Xnd(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
            Root::Instruction(Instruction::Xnd(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::And(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Orr(same_size2))]
          if same_size1 == same_size2
            && (resolve_node_value(&node1, &HashMap::new()).ok())
              .zip(resolve_node_value(&node2, &HashMap::new()).ok())
              .map(|(value1, value2)| value1 ^ value2 == 0xFF)
              .unwrap_or(false) =>
        {
          Some(vec![
            Root::Node(node2.clone()),
            Root::Instruction(Instruction::Orr(*same_size1)),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Orr(same_size1)), Root::Node(node2), Root::Instruction(Instruction::And(same_size2))]
          if same_size1 == same_size2
            && (resolve_node_value(&node1, &HashMap::new()).ok())
              .zip(resolve_node_value(&node2, &HashMap::new()).ok())
              .map(|(value1, value2)| value1 ^ value2 == 0xFF)
              .unwrap_or(false) =>
        {
          Some(vec![
            Root::Node(node2.clone()),
            Root::Instruction(Instruction::And(*same_size1)),
          ])
        }

        // `Node`s
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Add(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Sub(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Rot(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Rot(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Orr(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::And(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Xor(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Xnd(0x02))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }

        // `Ldo`s
        [Root::Node(node1), push_op1, push_op2, Root::Instruction(Instruction::Ldo(0x02))]
          if op_type(push_op1) == OpType::PushOp && op_type(push_op2) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(node1.clone()),
            push_op1.clone(),
            push_op2.clone(),
            Root::Node(node1.clone()),
          ])
        }

        _ => None,
      }
    });

    // length 5
    roots = match_replace(&roots, |window| match window {
      // `Ldo`s
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Instruction(Instruction::Ldo(0x03))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(node1.clone()),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          Root::Node(node1.clone()),
        ])
      }

      _ => None,
    });

    // length 6
    roots = match_replace(&roots, |window| match window {
      // `Node`s
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Add(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Sub(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Rot(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Rot(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Orr(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::And(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Xor(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Xnd(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }

      // `Ldo`s
      [Root::Node(node1), push_op1, push_op2, push_op3, push_op4, Root::Instruction(Instruction::Ldo(0x04))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && op_type(push_op4) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(node1.clone()),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          push_op4.clone(),
          Root::Node(node1.clone()),
        ])
      }

      _ => None,
    });
  }

  // optimize duplicate `Node`s (pushing them might take up two bytes) into `Ldo`s (always take up one byte)

  let mut last_roots = vec![];
  while roots != last_roots {
    last_roots = roots.clone();

    // length 2
    roots = match_replace(&roots, |window| match window {
      [Root::Node(same_node1), Root::Node(same_node2)] if same_node1 == same_node2 => Some(vec![
        Root::Node(same_node1.clone()),
        Root::Instruction(Instruction::Ldo(0x00)),
      ]),

      [Root::Instruction(Instruction::Swp), Root::Instruction(Instruction::Pop)] => {
        Some(vec![Root::Instruction(Instruction::Sto(0x00))])
      }

      _ => None,
    });

    // length 3
    roots = match_replace(&roots, |window| match window {
      [Root::Node(same_node1), push_op, Root::Node(same_node2)]
        if same_node1 == same_node2 && op_type(push_op) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(same_node1.clone()),
          push_op.clone(),
          Root::Instruction(Instruction::Ldo(0x01)),
        ])
      }

      _ => None,
    });

    // length 4
    roots = match_replace(&roots, |window| match window {
      [Root::Node(same_node1), push_op1, push_op2, Root::Node(same_node2)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(same_node1.clone()),
          push_op1.clone(),
          push_op2.clone(),
          Root::Instruction(Instruction::Ldo(0x02)),
        ])
      }

      _ => None,
    });

    // length 5
    roots = match_replace(&roots, |window| match window {
      [Root::Node(same_node1), push_op1, push_op2, push_op3, Root::Node(same_node2)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(same_node1.clone()),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          Root::Instruction(Instruction::Ldo(0x03)),
        ])
      }

      _ => None,
    });

    // length 6
    roots = match_replace(&roots, |window| match window {
      [Root::Node(same_node1), push_op1, push_op2, push_op3, push_op4, Root::Node(same_node2)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && op_type(push_op4) == OpType::PushOp =>
      {
        Some(vec![
          Root::Node(same_node1.clone()),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          push_op4.clone(),
          Root::Instruction(Instruction::Ldo(0x04)),
        ])
      }

      _ => None,
    });
  }

  roots
}

fn resolve_node_value(node: &Node, label_definitions: &HashMap<Label, u8>) -> Result<u8, Label> {
  // resolve `Node`s to `u8`s recursively while looking up `Label`s in `label_definitions`

  Ok(match node {
    Node::LabelRef(label) => *label_definitions.get(label).ok_or(label.clone())?,
    Node::Value(value) => *value,
    Node::Add(node1, node2) => resolve_node_value(node2, label_definitions)?
      .wrapping_add(resolve_node_value(node1, label_definitions)?),
    Node::Sub(node1, node2) => resolve_node_value(node2, label_definitions)?
      .wrapping_sub(resolve_node_value(node1, label_definitions)?),
    Node::Rot(node1, node2) => {
      let a = resolve_node_value(node1, label_definitions)? as u16;
      let b = resolve_node_value(node2, label_definitions)? as u16;
      let shifted = (b as u16) << a % 8;
      (shifted & 0xFF) as u8 | (shifted >> 8) as u8
    }
    Node::Orr(node1, node2) => {
      resolve_node_value(node2, label_definitions)? | resolve_node_value(node1, label_definitions)?
    }
    Node::And(node1, node2) => {
      resolve_node_value(node2, label_definitions)? & resolve_node_value(node1, label_definitions)?
    }
    Node::Xor(node1, node2) => {
      resolve_node_value(node2, label_definitions)? ^ resolve_node_value(node1, label_definitions)?
    }
    Node::Xnd(_node1, _node2) => 0,
    Node::Shl(node) => resolve_node_value(node, label_definitions)?.wrapping_shl(1),
    Node::Shr(node) => resolve_node_value(node, label_definitions)?.wrapping_shl(1),
    Node::Not(node) => !resolve_node_value(node, label_definitions)?,
  })
}
