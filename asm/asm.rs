use std::collections::{BTreeMap, HashMap, HashSet};

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

  let preprocessed: Vec<(Pos, String)> = preprocess(assembly_source_file, &mut errors, None);
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
  Conditional(Node, Node),
  LabelDefs(Vec<Label>),
  Node(Node),
  Const,
  Data(Option<Node>),
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

fn preprocess(
  file: File,
  errors: &mut impl Extend<(Pos, Error)>,
  pos: Option<Pos>,
) -> Vec<(Pos, String)> {
  // remove comments and resolve includes

  use std::path::Path;
  let assembly = std::fs::read_to_string(&file.0).unwrap_or_else(|_| {
    errors.extend([(
      pos.unwrap_or(Pos(File("[bootstrap]".to_string()), 0, 0)),
      Error(format!("Unable to read file `{}`", file)),
    )]);
    format!("")
  });

  let lines: Vec<(Pos, String)> = assembly
    .split("\n")
    .map(|line| line.strip_suffix("#").unwrap_or(line))
    .map(|line| line.split("# ").next().unwrap_or(line))
    .enumerate()
    .flat_map(|(row, line)| match line.find("@ ") {
      Some(col) => {
        let incl = File(
          Path::new(&file.0)
            .parent()
            .unwrap()
            .join(&line[col..]["@ ".len()..])
            .to_str()
            .unwrap()
            .to_string(),
        );
        std::iter::once((Pos(file.clone(), row, 0), line[..col].to_string()))
          .chain(preprocess(incl, errors, Some(Pos(file.clone(), row, col))))
          .collect::<Vec<_>>()
      }
      None => vec![(Pos(file.clone(), row, 0), line.to_string())],
    })
    .collect();

  lines
}

fn mnemonize(
  lines: Vec<(Pos, String)>,
  _errors: &mut impl Extend<(Pos, Error)>,
) -> Vec<(Pos, Mnemonic)> {
  let mnemonics: Vec<(Pos, Mnemonic)> = lines
    .into_iter()
    .flat_map(|(mut pos, line)| {
      let mut mnemonic = "".to_string();
      let mut mnemonics = vec![];
      for (col, char) in line.chars().enumerate() {
        if char.is_whitespace() {
          mnemonics.push((pos.clone(), mnemonic.clone()));
          mnemonic = "".to_string();
          pos = Pos(pos.0.clone(), pos.1, col + 1);
        } else {
          mnemonic.push(char);
        }
      }
      mnemonics.push((pos.clone(), mnemonic.clone()));
      mnemonics
    })
    .filter(|(_, mnemonic)| mnemonic.len() > 0)
    .map(|(pos, mnemonic)| (pos, Mnemonic(mnemonic)))
    .collect();

  mnemonics
}

fn tokenize(
  mnemonics: Vec<(Pos, Mnemonic)>,
  errors: &mut impl Extend<(Pos, Error)>,
) -> Vec<(Pos, Token)> {
  // tokenize to valid tokens. tokens might be invalid instructions

  let tokens: Vec<(Pos, Token)> = mnemonics
    .into_iter()
    .map(|(pos, mnemonic)| {
      (
        pos.clone(),
        match common::mnemonic_to_token(mnemonic.clone()) {
          Some(token) => token,
          None => {
            errors.extend([(
              pos.clone(),
              Error(format!("Invalid mnemonic `{}`", mnemonic)),
            )]);
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
  errors: &mut impl Extend<(Pos, Error)>,
  entry_point: &str,
) -> Vec<(Pos, Result<Instruction, u8>)> {
  // resolve macros recursively from `entry_point` and identify unused labels

  let mut macro_definitions: HashMap<Macro, Vec<(Pos, Token)>> = HashMap::new();
  let mut current_macro: Option<Macro> = None;

  for (pos, token) in tokens.into_iter() {
    match token {
      Token::MacroDef(r#macro) => {
        current_macro = Some(r#macro.clone());
        macro_definitions
          .entry(r#macro.clone())
          .and_modify(|_| {
            errors.extend([(
              pos.clone(),
              Error(format!("Duplicate macro definition `{}`", r#macro)),
            )]);
          })
          .or_insert(vec![]);
      }

      _ => match current_macro
        .as_ref()
        .and_then(|r#macro| macro_definitions.get_mut(&r#macro))
      {
        Some(macro_tokens) => macro_tokens.push((pos, token)),
        None => errors.extend([(pos, Error(format!("Orphan token `{}` encountered", token)))]),
      },
    }
  }

  let tokens = expand_macros(
    &vec![(
      Pos(File("[bootstrap]".to_string()), 0, 0),
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
    errors: &mut impl Extend<(Pos, Error)>,
  ) -> Vec<(Pos, Token)> {
    tokens
      .into_iter()
      .flat_map(|(pos, token)| match token {
        Token::MacroRef(r#macro) => {
          if parent_macros.contains(&r#macro) {
            errors.extend([(
              pos.clone(),
              Error(format!(
                "Macro self-reference {} -> `{}`",
                parent_macros
                  .iter()
                  .map(|r#macro| format!("`{}`", r#macro))
                  .collect::<Vec<String>>()
                  .join(" -> "),
                r#macro
              )),
            )]);
            return vec![];
          }

          let tokens = match macro_definitions.get(&r#macro) {
            Some(tokens) => tokens.clone(),
            None => {
              errors.extend([(
                pos.clone(),
                Error(format!("Reference to undefined macro `{}`", r#macro)),
              )]);
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
          parent_macros.push(r#macro.clone());
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

        Token::AtError => {
          errors.extend([(
            pos.clone(),
            Error(format!("`{}` directive encountered", token)),
          )]);
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
    (!label_references.contains(label)).then_some((
      pos.clone(),
      Error(format!("Unused label definition `{}`", label)),
    ))
  }));

  // turn assembly tokens into roots, an intermediate representation for optimization. roots correspond to valid instructions

  let roots: Vec<(Pos, Root)> = tokens
    .into_iter()
    .map(|(pos, token)| {
      let token = match token {
        Token::LabelDef(label) => Root::LabelDefs(vec![label]),
        Token::LabelRef(label) => Root::Node(Node::LabelRef(label)),
        Token::MacroDef(_) => panic!("Macro definition found in intermediate representation"),
        Token::MacroRef(_) => panic!("Macro reference found in intermediate representation"),
        Token::AtError => panic!("Error directive found in intermediate representation"),
        Token::AtConst => Root::Const,
        Token::AtData => Root::Data(None),
        Token::AtDyn => Root::Dyn(None),
        Token::AtOrg => Root::Org(None),
        Token::XXX(value) => Root::Node(Node::Value(value)),
        Token::Add => Root::Instruction(Instruction::Add(Size::assert(0x01))),
        Token::AdS(size) => Root::Instruction(Instruction::Add(size)),
        Token::Sub => Root::Instruction(Instruction::Sub(Size::assert(0x01))),
        Token::SuS(size) => Root::Instruction(Instruction::Sub(size)),
        Token::Iff => Root::Instruction(Instruction::Iff(Size::assert(0x01))),
        Token::IfS(size) => Root::Instruction(Instruction::Iff(size)),
        Token::Swp => Root::Instruction(Instruction::Swp(Size::assert(0x01))),
        Token::SwS(size) => Root::Instruction(Instruction::Swp(size)),
        Token::Rot => Root::Instruction(Instruction::Rot(Size::assert(0x01))),
        Token::RoS(size) => Root::Instruction(Instruction::Rot(size)),
        Token::Orr => Root::Instruction(Instruction::Orr(Size::assert(0x01))),
        Token::OrS(size) => Root::Instruction(Instruction::Orr(size)),
        Token::And => Root::Instruction(Instruction::And(Size::assert(0x01))),
        Token::AnS(size) => Root::Instruction(Instruction::And(size)),
        Token::Xor => Root::Instruction(Instruction::Xor(Size::assert(0x01))),
        Token::XoS(size) => Root::Instruction(Instruction::Xor(size)),
        Token::Xnd => Root::Instruction(Instruction::Xnd(Size::assert(0x01))),
        Token::XnS(size) => Root::Instruction(Instruction::Xnd(size)),
        Token::Inc => Root::Instruction(Instruction::Inc),
        Token::Dec => Root::Instruction(Instruction::Dec),
        Token::Neg => Root::Instruction(Instruction::Neg),
        Token::Shl => Root::Instruction(Instruction::Shl),
        Token::Shr => Root::Instruction(Instruction::Shr),
        Token::Not => Root::Instruction(Instruction::Not),
        Token::Buf => Root::Instruction(Instruction::Buf),
        Token::LdO(ofst) => Root::Instruction(Instruction::Ldo(ofst)),
        Token::StO(ofst) => Root::Instruction(Instruction::Sto(ofst)),
        Token::Lda => Root::Instruction(Instruction::Lda),
        Token::Sta => Root::Instruction(Instruction::Sta),
        Token::Ldi => Root::Instruction(Instruction::Ldi),
        Token::Sti => Root::Instruction(Instruction::Sti),
        Token::Lds => Root::Instruction(Instruction::Lds),
        Token::Sts => Root::Instruction(Instruction::Sts),
        Token::Clc => Root::Instruction(Instruction::Clc),
        Token::Sec => Root::Instruction(Instruction::Sec),
        Token::Flc => Root::Instruction(Instruction::Flc),
        Token::Nop => Root::Instruction(Instruction::Nop),
        Token::Pop => Root::Instruction(Instruction::Pop),
        Token::AtDD(0xBB) => Root::Instruction(Instruction::Dbg),
        Token::AtDD(value) => Root::Data(Some(Node::Value(value))),
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

    let instructions = match value {
      0b11110000..=0b11111111 => vec![Instruction::Phn(Nimm::assert(value))],
      0b10000000..=0b10000000 => vec![
        Instruction::Psh(Imm::assert(value.wrapping_sub(1))),
        Instruction::Inc,
      ],
      0b00000000..=0b01111111 => vec![(Instruction::Psh(Imm::assert(value)))],
      0b10000000..=0b11111111 => vec![
        Instruction::Psh(Imm::assert(value.wrapping_neg())),
        Instruction::Neg,
      ],
    };

    instructions
      .into_iter()
      .map(|instruction| (pos.clone(), instruction))
      .collect()
  }

  // if every label a node depends on could be resolved, we can replace it with a value.
  // if not, start by allocating one byte for pushing the node later. if pushing the node turns
  // out to require more than one byte, iteratively `'bruteforce` allocation sizes until we
  // find one that works. repeat for every node.

  let mut instructions: Vec<(Pos, Result<Instruction, u8>)>;
  let mut allocation_sizes: HashMap<Node, usize> = HashMap::new();
  let mut bruteforce_errors: Vec<(Pos, Error)> = vec![];

  macro_rules! allocation_size {
    ($node:expr) => {
      allocation_sizes.get($node).copied().unwrap_or(1)
    };
  }

  'bruteforce: loop {
    let mut location_counter: usize = 0;
    let mut label_definitions: HashMap<Label, u8> = HashMap::new();
    let mut unevaluated_nodes: BTreeMap<u8, (Pos, Node)> = BTreeMap::new();

    instructions = roots
      .iter()
      .flat_map(|(pos, root)| {
        let instructions = match root {
          Root::Instruction(instruction) | Root::Dyn(Some(instruction)) => {
            vec![(pos.clone(), Ok(instruction.clone()))]
          }

          Root::Conditional(node1, node2) => {
            let mut node1 = node1.clone();
            let mut node2 = node2.clone();
            let mut instructions = vec![];
            if allocation_size!(&node1) > 1 && allocation_size!(&node2) > 1 {
              // if both arguments of a conditional can only be pushed indirectly, negate both nodes
              // so they can both be pushed directly, then negate result of the conditional itself.
              // this saves one byte over emitting as-is
              node1 = Node::Sub(Box::new(node1), Box::new(Node::Value(0x00)));
              node2 = Node::Sub(Box::new(node2), Box::new(Node::Value(0x00)));
              instructions.extend(vec![
                (pos.clone(), Ok(Instruction::Nop));
                allocation_size!(&node1) + allocation_size!(&node2)
              ]);
              instructions.extend(vec![
                (pos.clone(), Ok(Instruction::Iff(Size::assert(0x01)))),
                (pos.clone(), Ok(Instruction::Neg)),
              ]);
            } else {
              // else, if at least one argument can be pushed directly, emit as-is.
              // there is no byte to be saved here
              instructions.extend(vec![
                (pos.clone(), Ok(Instruction::Nop));
                allocation_size!(&node1) + allocation_size!(&node2)
              ]);
              instructions.extend(vec![(
                pos.clone(),
                Ok(Instruction::Iff(Size::assert(0x01))),
              )]);
            }
            unevaluated_nodes.insert(location_counter as u8, (pos.clone(), node1.clone()));
            unevaluated_nodes.insert(
              (location_counter + allocation_size!(&node1)) as u8,
              (pos.clone(), node2.clone()),
            );
            instructions
          }

          Root::LabelDefs(labels) => {
            labels.iter().for_each(|label| {
              if matches!(label, Label::Local(_, None)) {
                panic!("Local label has no scope specified")
              }

              if label_definitions.contains_key(&label) {
                bruteforce_errors.extend([(
                  pos.clone(),
                  Error(format!("Duplicate label definition `{}`", label)),
                )]);
              }
              label_definitions.insert(label.clone(), location_counter as u8);
            });
            vec![]
          }

          Root::Node(node) => match resolve_node_value(&node, &label_definitions) {
            Ok(value) => build_push_instruction(value, &pos)
              .into_iter()
              .map(|(pos, instruction)| (pos, Ok(instruction)))
              .collect::<Vec<_>>(),
            Err(_) => {
              unevaluated_nodes.insert(location_counter as u8, (pos.clone(), node.clone()));
              vec![(pos.clone(), Ok(Instruction::Nop)); allocation_size!(&node)]
            }
          },

          Root::Const => {
            bruteforce_errors.extend([(
              pos.clone(),
              Error(format!(
                "`{}` argument could not be reduced to a constant expression",
                Token::AtConst,
              )),
            )]);
            vec![]
          }

          Root::Data(Some(node)) => match resolve_node_value(&node, &label_definitions) {
            Ok(value) => vec![(pos.clone(), Err(value))],
            Err(label) => {
              bruteforce_errors.extend([(
                pos.clone(),
                Error(format!(
                  "`{}` argument references currently unresolved label `{}`",
                  Token::AtData,
                  label
                )),
              )]);
              vec![]
            }
          },

          Root::Data(None) => {
            bruteforce_errors.extend([(
              pos.clone(),
              Error(format!(
                "`{}` argument could not be reduced to a constant expression",
                Token::AtData,
              )),
            )]);
            vec![]
          }

          Root::Dyn(None) => {
            bruteforce_errors.extend([(
              pos.clone(),
              Error(format!(
                "`{}` argument could not be reduced to an instruction",
                Token::AtDyn,
              )),
            )]);
            vec![]
          }

          Root::Org(Some(node)) => match resolve_node_value(&node, &label_definitions) {
            Ok(value) => match (value as usize).checked_sub(location_counter) {
              Some(padding) => {
                vec![(pos.clone(), Err(0x00)); padding]
              }
              None => {
                bruteforce_errors.extend([(
                  pos.clone(),
                  Error(format!(
                    "`{}` cannot move location counter backward from `{:02X}` to `{:02X}`",
                    Token::AtOrg,
                    location_counter,
                    value
                  )),
                )]);
                vec![]
              }
            },
            Err(label) => {
              bruteforce_errors.extend([(
                pos.clone(),
                Error(format!(
                  "`{}` argument references currently unresolved label `{}`",
                  Token::AtOrg,
                  label
                )),
              )]);
              vec![]
            }
          },

          Root::Org(None) => {
            bruteforce_errors.extend([(
              pos.clone(),
              Error(format!(
                "`{}` argument could not be reduced to a constant expression",
                Token::AtOrg,
              )),
            )]);
            vec![]
          }
        };
        location_counter += instructions.len();
        instructions
      })
      .collect();

    // poke into `instructions` and evaluate the nodes that couldn't be evaluated before
    'poke: {
      for (location_counter, (pos, node)) in unevaluated_nodes.iter() {
        let value = match resolve_node_value(&node, &label_definitions) {
          Ok(value) => value,
          Err(label) => {
            bruteforce_errors.extend([(
              pos.clone(),
              Error(format!("Reference to undefined label `{}`", label)),
            )]);
            0x00
          }
        };

        // if the evaluated node doesn't fit in the allocated memory, note down the right amount of
        // memory to allocate on the next iteration of `'bruteforce` and try again

        let push_instructions = build_push_instruction(value, &pos);
        if push_instructions.len() > allocation_size!(&node) {
          allocation_sizes.insert(node.clone(), push_instructions.len());
          break 'poke;
        }

        for (index, (pos, instruction)) in push_instructions.into_iter().enumerate() {
          instructions[*location_counter as usize + index] = (pos, Ok(instruction));
        }
      }

      // all unevaluated nodes have been evaluated, break out of the bruteforce loop
      break 'bruteforce;
    }

    // abort brute force if errors were encountered
    if bruteforce_errors.len() > 0 {
      break 'bruteforce;
    }
  }

  errors.extend(bruteforce_errors);

  instructions
}

fn codegen(
  instructions: Vec<(Pos, Result<Instruction, u8>)>,
  errors: &mut impl Extend<(Pos, Error)>,
) -> Vec<(Pos, u8)> {
  // codegen instructions into opcodes

  let opcodes: Vec<(Pos, u8)> = instructions
    .into_iter()
    .map(|(pos, instruction)| (pos, common::instruction_to_opcode(instruction)))
    .collect();

  let mut opcodes = opcodes;
  let pos = Pos(File("[codegen]".to_string()), 0, 0);

  match common::MEM_SIZE.checked_sub(opcodes.len()) {
    Some(padding) => opcodes.extend(vec![(pos, 0x00); padding]),
    None => {
      errors.extend([(
        pos,
        Error(format!(
          "Program size `{:02X}` exceeds available memory of size `{:02X}`",
          opcodes.len(),
          common::MEM_SIZE
        )),
      )]);
    }
  }

  opcodes
}

fn optimize(roots: Vec<(Pos, Root)>, _errors: &mut impl Extend<(Pos, Error)>) -> Vec<(Pos, Root)> {
  // build a tree of nodes representing everything we can compute at compile time
  // this removes redundant instructions and makes macros usable

  // a convenience function to replace slice patterns within a vector
  fn match_replace<const N: usize>(
    roots: &Vec<(Pos, Root)>,
    mut replacer: impl FnMut(&[Root; N]) -> Option<Vec<Root>>,
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
        Instruction::Add(Size(0x01)) => OpType::BinaryOp,
        Instruction::Add(_size) => OpType::Impure,
        Instruction::Sub(Size(0x01)) => OpType::BinaryOp,
        Instruction::Sub(_size) => OpType::Impure,
        Instruction::Iff(Size(0x01)) => OpType::BinaryOp,
        Instruction::Iff(_size) => OpType::Impure,
        Instruction::Swp(Size(0x01)) => OpType::DualOp,
        Instruction::Swp(_size) => OpType::Impure,
        Instruction::Rot(Size(0x01)) => OpType::BinaryOp,
        Instruction::Rot(_size) => OpType::Impure,
        Instruction::Orr(Size(0x01)) => OpType::BinaryOp,
        Instruction::Orr(_size) => OpType::Impure,
        Instruction::And(Size(0x01)) => OpType::BinaryOp,
        Instruction::And(_size) => OpType::Impure,
        Instruction::Xor(Size(0x01)) => OpType::BinaryOp,
        Instruction::Xor(_size) => OpType::Impure,
        Instruction::Xnd(Size(0x01)) => OpType::BinaryOp,
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
        Instruction::Pop => OpType::PopOp,
        Instruction::Phn(_nimm) => OpType::PushOp,
      },
      Root::Conditional(_, _) => OpType::PushOp,
      Root::LabelDefs(_) => OpType::Impure,
      Root::Node(_) => OpType::PushOp,
      Root::Const => OpType::Impure,
      Root::Data(_) => OpType::Impure,
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

    // higher priority for directives
    roots = match_replace(&roots, |window| match window {
      [node @ Root::Node(_), Root::Const] => Some(vec![node.clone()]),
      [Root::Instruction(instruction), Root::Dyn(None)] => {
        Some(vec![Root::Dyn(Some(instruction.clone()))])
      }
      [r#dyn @ Root::Dyn(Some(_)), Root::Dyn(None)] => Some(vec![r#dyn.clone()]),
      [Root::Node(Node::Value(value)), Root::Dyn(None)] => {
        match common::opcode_to_instruction(*value) {
          Ok(instruction @ Instruction::Psh(_)) => Some(vec![Root::Dyn(Some(instruction))]),
          Ok(instruction @ Instruction::Phn(_)) => Some(vec![Root::Dyn(Some(instruction))]),
          _ => None,
        }
      }
      [Root::Node(node), Root::Data(None)] => Some(vec![Root::Data(Some(node.clone()))]),
      [Root::Node(node), Root::Org(None)] => Some(vec![Root::Org(Some(node.clone()))]),
      _ => None,
    });

    // for `!pad` macro
    roots = match_replace(&roots, |window| match window {
      [node @ Root::Node(_), label_defs @ Root::LabelDefs(_), r#const @ Root::Const] => {
        Some(vec![node.clone(), r#const.clone(), label_defs.clone()])
      }
      [node @ Root::Node(_), label_defs @ Root::LabelDefs(_), data @ Root::Data(None)] => {
        Some(vec![node.clone(), data.clone(), label_defs.clone()])
      }
      [node @ Root::Node(_), label_defs @ Root::LabelDefs(_), org @ Root::Org(None)] => {
        Some(vec![label_defs.clone(), node.clone(), org.clone()])
      }
      _ => None,
    });

    // for patterns such as `:label1 !bcs :label2 !jmp`
    let mut label_aliases: HashMap<Label, Vec<Label>> = HashMap::new();
    roots = match_replace(&roots, |window| match window {
      [Root::LabelDefs(diff_labels), Root::Node(Node::LabelRef(diff_label)), Root::Instruction(Instruction::Sti)]
        if !diff_labels.contains(&diff_label) =>
      {
        label_aliases
          .entry(diff_label.clone())
          .or_insert_with(|| vec![])
          .extend(diff_labels.clone());
        Some(vec![])
      }
      _ => None,
    });
    roots = match_replace(&roots, |window| match window {
      [Root::LabelDefs(labels)] => Some(vec![Root::LabelDefs(
        labels
          .iter()
          .flat_map(|label| {
            std::iter::once(label.clone())
              .chain(label_aliases.get(label).cloned().unwrap_or(vec![]))
          })
          .collect(),
      )]),

      _ => None,
    });

    // length 1
    roots = match_replace(&roots, |window| match window {
      // `OpType`s
      [no_op] if op_type(no_op) == OpType::NoOp => Some(vec![]),

      _ => None,
    });

    // length 2
    roots = match_replace(&roots, |window| match window {
      // `Node`s
      [Root::Node(x00), Root::Instruction(Instruction::Add(_size))]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![])
      }
      [Root::Node(x01), Root::Instruction(Instruction::Add(Size(0x01)))]
        if resolve_node_value(&x01, &HashMap::new()) == Ok(0x01) =>
      {
        Some(vec![Root::Instruction(Instruction::Inc)])
      }
      [Root::Node(x00), Root::Instruction(Instruction::Sub(_size))]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![])
      }
      [Root::Node(x01), Root::Instruction(Instruction::Sub(Size(0x01)))]
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
      [Root::Node(xff), Root::Instruction(Instruction::And(_size))]
        if resolve_node_value(&xff, &HashMap::new()) == Ok(0xFF) =>
      {
        Some(vec![])
      }
      [Root::Node(x00), Root::Instruction(Instruction::Xor(_size))]
        if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
      {
        Some(vec![])
      }
      [Root::Node(node), Root::Instruction(Instruction::Inc)] => Some(vec![Root::Node(Node::Add(
        Box::new(Node::Value(0x01)),
        Box::new(node.clone()),
      ))]),

      [Root::Node(node), Root::Instruction(Instruction::Dec)] => Some(vec![Root::Node(Node::Sub(
        Box::new(Node::Value(0x01)),
        Box::new(node.clone()),
      ))]),

      [Root::Node(node), Root::Instruction(Instruction::Neg)] => Some(vec![Root::Node(Node::Sub(
        Box::new(node.clone()),
        Box::new(Node::Value(0x00)),
      ))]),
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

      // `Ldo`s
      [node @ Root::Node(_), Root::Instruction(Instruction::Ldo(Ofst(0x00)))] => {
        Some(vec![node.clone(), node.clone()])
      }
      [Root::Instruction(Instruction::Ldo(same_ofst1)), Root::Instruction(Instruction::Sto(same_ofst2))]
        if same_ofst1 == same_ofst2 =>
      {
        Some(vec![])
      }

      // idempotent and involutive `UnaryOp`s
      [Root::Instruction(Instruction::Swp(same_size1)), Root::Instruction(Instruction::Swp(same_size2))]
        if same_size1 == same_size2 =>
      {
        Some(vec![])
      }
      [clc @ Root::Instruction(Instruction::Clc), Root::Instruction(Instruction::Clc)] => {
        Some(vec![clc.clone()])
      }
      [sec @ Root::Instruction(Instruction::Sec), Root::Instruction(Instruction::Sec)] => {
        Some(vec![sec.clone()])
      }
      [Root::Instruction(Instruction::Flc), Root::Instruction(Instruction::Flc)] => Some(vec![]),

      //  `Label`s
      [Root::LabelDefs(labels1), Root::LabelDefs(labels2)] => Some(vec![Root::LabelDefs(
        labels1.iter().chain(labels2.iter()).cloned().collect(),
      )]),

      // `OpType`s
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
    roots = match_replace(&roots, |window| {
      match window {
        // `Conditional`s
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Iff(Size(0x01)))] => {
          Some(vec![Root::Conditional(node1.clone(), node2.clone())])
        }

        // `Node`s
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Add(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Add(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Sub(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Sub(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Rot(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Rot(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Orr(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Orr(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::And(Size(0x01)))] => {
          Some(vec![Root::Node(Node::And(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xor(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Xor(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }
        [Root::Node(node1), Root::Node(node2), Root::Instruction(Instruction::Xnd(Size(0x01)))] => {
          Some(vec![Root::Node(Node::Xnd(
            Box::new(node2.clone()),
            Box::new(node1.clone()),
          ))])
        }

        // `Swp`s
        [Root::Instruction(Instruction::Swp(Size(0x01))), Root::Instruction(Instruction::Inc), Root::Instruction(Instruction::Swp(Size(0x01)))] => {
          Some(vec![
            Root::Node(Node::Value(0x01)),
            Root::Instruction(Instruction::Add(Size::assert(0x02))),
          ])
        }
        [Root::Instruction(Instruction::Swp(Size(0x01))), Root::Instruction(Instruction::Dec), Root::Instruction(Instruction::Swp(Size(0x01)))] => {
          Some(vec![
            Root::Node(Node::Value(0x01)),
            Root::Instruction(Instruction::Sub(Size::assert(0x02))),
          ])
        }
        [node1 @ Root::Node(_), node2 @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x01)))] => {
          Some(vec![node2.clone(), node1.clone()])
        }

        [Root::Instruction(Instruction::Ldo(Ofst(ofst))), node @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x01)))]
          if ofst.checked_add(1).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            node.clone(),
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst + 1))),
          ])
        }

        [node @ Root::Node(_), Root::Instruction(Instruction::Ldo(Ofst(ofst))), Root::Instruction(Instruction::Swp(Size(0x01)))]
          if ofst.checked_sub(1).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst - 1))),
            node.clone(),
          ])
        }
        [Root::Instruction(Instruction::Ldo(Ofst(ofst1))), Root::Instruction(Instruction::Ldo(Ofst(ofst2))), Root::Instruction(Instruction::Swp(Size(0x01)))]
          if ofst1.checked_add(1).and_then(Ofst::new).is_some()
            && ofst2.checked_sub(1).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst2 - 1))),
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst1 + 1))),
          ])
        }

        // `Ldo`s
        [node @ Root::Node(_), push_op, Root::Instruction(Instruction::Ldo(Ofst(0x01)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![node.clone(), push_op.clone(), node.clone()])
        }

        // `Sto`s
        [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x07)))]
        | [Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x08))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          // OpType::PopOp
          Some(vec![Root::Instruction(Instruction::Xnd(Size::assert(
            0x08,
          )))])
        }
        [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x03)))]
        | [Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x04))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          // OpType::PopOp
          Some(vec![Root::Instruction(Instruction::Xnd(Size::assert(
            0x04,
          )))])
        }
        [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x01)))]
        | [Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x02))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          // OpType::PopOp
          Some(vec![Root::Instruction(Instruction::Xnd(Size::assert(
            0x02,
          )))])
        }
        [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x00)]
        | [Root::Instruction(Instruction::Pop), Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x00)))]
        | [Root::Node(x00), Root::Instruction(Instruction::Sto(Ofst(0x01))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x00, &HashMap::new()) == Ok(0x00) =>
        {
          // OpType::PopOp
          Some(vec![Root::Instruction(Instruction::Xnd(Size::assert(
            0x01,
          )))])
        }
        [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x01)]
        | [Root::Instruction(Instruction::Pop), Root::Node(x01), Root::Instruction(Instruction::Sto(Ofst(0x00)))]
        | [Root::Node(x01), Root::Instruction(Instruction::Sto(Ofst(0x01))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x01, &HashMap::new()) == Ok(0x01) =>
        {
          // OpType::PopOp
          Some(vec![
            Root::Instruction(Instruction::Xnd(Size::assert(0x01))),
            Root::Instruction(Instruction::Shl),
          ])
        }
        [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(x80)]
        | [Root::Instruction(Instruction::Pop), Root::Node(x80), Root::Instruction(Instruction::Sto(Ofst(0x00)))]
        | [Root::Node(x80), Root::Instruction(Instruction::Sto(Ofst(0x01))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&x80, &HashMap::new()) == Ok(0x80) =>
        {
          // OpType::PopOp
          Some(vec![
            Root::Instruction(Instruction::Xnd(Size::assert(0x01))),
            Root::Instruction(Instruction::Shr),
          ])
        }
        [Root::Instruction(Instruction::Pop), Root::Instruction(Instruction::Pop), Root::Node(xff)]
        | [Root::Instruction(Instruction::Pop), Root::Node(xff), Root::Instruction(Instruction::Sto(Ofst(0x00)))]
        | [Root::Node(xff), Root::Instruction(Instruction::Sto(Ofst(0x01))), Root::Instruction(Instruction::Pop)]
          if resolve_node_value(&xff, &HashMap::new()) == Ok(0xFF) =>
        {
          // OpType::PopOp
          Some(vec![
            Root::Instruction(Instruction::Xnd(Size::assert(0x01))),
            Root::Instruction(Instruction::Not),
          ])
        }

        // for `cc` macro return and if statement codegen
        [Root::Node(Node::LabelRef(same_label)), Root::Instruction(Instruction::Sti), Root::LabelDefs(same_labels)]
          if same_labels.contains(&same_label) =>
        {
          Some(vec![Root::LabelDefs(same_labels.clone())])
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
      }
    });

    // length 4
    roots = match_replace(&roots, |window| {
      match window {
        // doubled `BinaryOp`s
        [Root::Node(node1), and @ Root::Instruction(Instruction::Add(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Add(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            and.clone(),
          ])
        }
        [Root::Node(node1), add @ Root::Instruction(Instruction::Add(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Sub(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            add.clone(),
          ])
        }
        [Root::Node(node1), sub @ Root::Instruction(Instruction::Sub(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Sub(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            sub.clone(),
          ])
        }
        [Root::Node(node1), sub @ Root::Instruction(Instruction::Sub(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Add(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            sub.clone(),
          ])
        }
        [Root::Node(node1), rot @ Root::Instruction(Instruction::Rot(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Rot(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            rot.clone(),
          ])
        }
        [Root::Node(node1), orr @ Root::Instruction(Instruction::Orr(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Orr(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
            orr.clone(),
          ])
        }
        [Root::Node(node1), and @ Root::Instruction(Instruction::And(same_size1)), Root::Node(node2), Root::Instruction(Instruction::And(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
            and.clone(),
          ])
        }
        [Root::Node(node1), xor @ Root::Instruction(Instruction::Xor(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Xor(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
            xor.clone(),
          ])
        }
        [Root::Node(node1), xnd @ Root::Instruction(Instruction::Xnd(same_size1)), Root::Node(node2), Root::Instruction(Instruction::Xnd(same_size2))]
          if same_size1 == same_size2 =>
        {
          Some(vec![
            Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
            xnd.clone(),
          ])
        }
        [Root::Node(node1), Root::Instruction(Instruction::And(same_size1)), Root::Node(node2), orr @ Root::Instruction(Instruction::Orr(same_size2))]
          if same_size1 == same_size2
            && (resolve_node_value(&node1, &HashMap::new()).ok())
              .zip(resolve_node_value(&node2, &HashMap::new()).ok())
              .map(|(value1, value2)| value1 ^ value2 == 0xFF)
              .unwrap_or(false) =>
        {
          Some(vec![Root::Node(node2.clone()), orr.clone()])
        }
        [Root::Node(node1), Root::Instruction(Instruction::Orr(same_size1)), Root::Node(node2), and @ Root::Instruction(Instruction::And(same_size2))]
          if same_size1 == same_size2
            && (resolve_node_value(&node1, &HashMap::new()).ok())
              .zip(resolve_node_value(&node2, &HashMap::new()).ok())
              .map(|(value1, value2)| value1 ^ value2 == 0xFF)
              .unwrap_or(false) =>
        {
          Some(vec![Root::Node(node2.clone()), and.clone()])
        }

        // `Conditional`s
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Iff(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Conditional(node1.clone(), node2.clone()),
            push_op.clone(),
          ])
        }

        // `Node`s
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Add(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Add(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Sub(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Sub(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Rot(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Rot(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Orr(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Orr(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::And(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::And(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Xor(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Xor(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }
        [Root::Node(node1), push_op, Root::Node(node2), Root::Instruction(Instruction::Xnd(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![
            Root::Node(Node::Xnd(Box::new(node2.clone()), Box::new(node1.clone()))),
            push_op.clone(),
          ])
        }

        // `Swp`s
        [node1 @ Root::Node(_), push_op, node2 @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp =>
        {
          Some(vec![node2.clone(), push_op.clone(), node1.clone()])
        }
        [Root::Instruction(Instruction::Ldo(Ofst(ofst))), push_op, node @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp
            && ofst.checked_add(2).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            node.clone(),
            push_op.clone(),
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst + 2))),
          ])
        }
        [node @ Root::Node(_), push_op, Root::Instruction(Instruction::Ldo(Ofst(ofst))), Root::Instruction(Instruction::Swp(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp
            && ofst.checked_sub(2).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst - 2))),
            push_op.clone(),
            node.clone(),
          ])
        }
        [Root::Instruction(Instruction::Ldo(Ofst(ofst1))), push_op, Root::Instruction(Instruction::Ldo(Ofst(ofst2))), Root::Instruction(Instruction::Swp(Size(0x02)))]
          if op_type(push_op) == OpType::PushOp
            && ofst1.checked_add(2).and_then(Ofst::new).is_some()
            && ofst2.checked_sub(2).and_then(Ofst::new).is_some() =>
        {
          Some(vec![
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst2 - 2))),
            push_op.clone(),
            Root::Instruction(Instruction::Ldo(Ofst::assert(ofst1 + 2))),
          ])
        }

        // `Ldo`s
        [node @ Root::Node(_), push_op1, push_op2, Root::Instruction(Instruction::Ldo(Ofst(0x02)))]
          if op_type(push_op1) == OpType::PushOp && op_type(push_op2) == OpType::PushOp =>
        {
          Some(vec![
            node.clone(),
            push_op1.clone(),
            push_op2.clone(),
            node.clone(),
          ])
        }

        _ => None,
      }
    });

    // length 5
    roots = match_replace(&roots, |window| match window {
      // `Ldo`s
      [node @ Root::Node(_), push_op1, push_op2, push_op3, Root::Instruction(Instruction::Ldo(Ofst(0x03)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          node.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          node.clone(),
        ])
      }

      _ => None,
    });

    // length 6
    roots = match_replace(&roots, |window| match window {
      // `Conditional`s
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Iff(Size(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          Root::Conditional(node1.clone(), node2.clone()),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
        ])
      }

      // `Node`s
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Add(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Sub(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Rot(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Orr(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::And(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Xor(Size(0x04)))]
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
      [Root::Node(node1), push_op1, push_op2, push_op3, Root::Node(node2), Root::Instruction(Instruction::Xnd(Size(0x04)))]
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

      // `Swp`s
      [node1 @ Root::Node(_), push_op1, push_op2, push_op3, node2 @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          node2.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          node1.clone(),
        ])
      }
      [Root::Instruction(Instruction::Ldo(Ofst(ofst))), push_op1, push_op2, push_op3, node @ Root::Node(_), Root::Instruction(Instruction::Swp(Size(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && ofst.checked_add(4).and_then(Ofst::new).is_some() =>
      {
        Some(vec![
          node.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(ofst + 4))),
        ])
      }
      [node @ Root::Node(_), push_op1, push_op2, push_op3, Root::Instruction(Instruction::Ldo(Ofst(ofst))), Root::Instruction(Instruction::Swp(Size(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && ofst.checked_sub(4).and_then(Ofst::new).is_some() =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(Ofst::assert(ofst - 4))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          node.clone(),
        ])
      }
      [Root::Instruction(Instruction::Ldo(Ofst(ofst1))), push_op1, push_op2, push_op3, Root::Instruction(Instruction::Ldo(Ofst(ofst2))), Root::Instruction(Instruction::Swp(Size(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && ofst1.checked_add(4).and_then(Ofst::new).is_some()
          && ofst2.checked_sub(4).and_then(Ofst::new).is_some() =>
      {
        Some(vec![
          Root::Instruction(Instruction::Ldo(Ofst::assert(ofst2 - 4))),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(ofst1 + 4))),
        ])
      }

      // `Ldo`s
      [node @ Root::Node(_), push_op1, push_op2, push_op3, push_op4, Root::Instruction(Instruction::Ldo(Ofst(0x04)))]
        if op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && op_type(push_op4) == OpType::PushOp =>
      {
        Some(vec![
          node.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          push_op4.clone(),
          node.clone(),
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
      [same_node1 @ Root::Node(_), same_node2 @ Root::Node(_)] if same_node1 == same_node2 => {
        Some(vec![
          same_node1.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(0x00))),
        ])
      }
      [Root::Instruction(Instruction::Swp(Size(size))), Root::Instruction(Instruction::Pop)]
        if size.checked_sub(1).and_then(Ofst::new).is_some() =>
      {
        Some(vec![Root::Instruction(Instruction::Sto(Ofst::assert(
          size - 1,
        )))])
      }
      _ => None,
    });

    // length 3
    roots = match_replace(&roots, |window| match window {
      [same_node1 @ Root::Node(_), push_op, same_node2 @ Root::Node(_)]
        if same_node1 == same_node2 && op_type(push_op) == OpType::PushOp =>
      {
        Some(vec![
          same_node1.clone(),
          push_op.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(0x01))),
        ])
      }
      _ => None,
    });

    // length 4
    roots = match_replace(&roots, |window| match window {
      [same_node1 @ Root::Node(_), push_op1, push_op2, same_node2 @ Root::Node(_)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp =>
      {
        Some(vec![
          same_node1.clone(),
          push_op1.clone(),
          push_op2.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(0x02))),
        ])
      }
      _ => None,
    });

    // length 5
    roots = match_replace(&roots, |window| match window {
      [same_node1 @ Root::Node(_), push_op1, push_op2, push_op3, same_node2 @ Root::Node(_)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp =>
      {
        Some(vec![
          same_node1.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(0x03))),
        ])
      }
      _ => None,
    });

    // length 6
    roots = match_replace(&roots, |window| match window {
      [same_node1 @ Root::Node(_), push_op1, push_op2, push_op3, push_op4, same_node2 @ Root::Node(_)]
        if same_node1 == same_node2
          && op_type(push_op1) == OpType::PushOp
          && op_type(push_op2) == OpType::PushOp
          && op_type(push_op3) == OpType::PushOp
          && op_type(push_op4) == OpType::PushOp =>
      {
        Some(vec![
          same_node1.clone(),
          push_op1.clone(),
          push_op2.clone(),
          push_op3.clone(),
          push_op4.clone(),
          Root::Instruction(Instruction::Ldo(Ofst::assert(0x04))),
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
    Node::Shr(node) => resolve_node_value(node, label_definitions)?.wrapping_shr(1),
    Node::Not(node) => !resolve_node_value(node, label_definitions)?,
  })
}
