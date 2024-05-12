use crate::*;
use std::collections::HashSet;

pub fn optimize(program: TypedProgram, _errors: &mut impl Extend<(Pos, Error)>) -> TypedProgram {
  optimize::program(program)
}

fn program(program: TypedProgram) -> TypedProgram {
  match program {
    TypedProgram(globals) => TypedProgram(globals.into_iter().map(optimize::global).collect()),
  }
}

fn global(global: TypedGlobal) -> TypedGlobal {
  match global {
    TypedGlobal::Data(label, value) => {
      TypedGlobal::Data(label, value.into_iter().map(optimize::expression).collect())
    }

    TypedGlobal::Macro(label, body, return_template) => {
      TypedGlobal::Macro(label, optimize::statement(body), return_template)
    }

    TypedGlobal::Function(label, body, return_template) => {
      TypedGlobal::Function(label, optimize::statement(body), return_template)
    }

    TypedGlobal::Assembly(assembly) => TypedGlobal::Assembly(assembly),
  }
}

fn statement(statement: TypedStatement) -> TypedStatement {
  if statement_behavior(&statement).is_none() {
    // behavior is undefined and therefore statement shall not be executed
    return TypedStatement::Compound(vec![]);
  }

  match statement {
    TypedStatement::ExpressionN0(expression) => {
      TypedStatement::ExpressionN0(optimize::expression(expression))
    }

    TypedStatement::Compound(statements) => {
      TypedStatement::Compound(
        statements
          .iter()
          .cloned()
          .zip(std::iter::once(TypedStatement::Compound(vec![])).chain(statements.iter().cloned()))
          .take_while(|(_statement, prev)| {
            statement_behavior(prev)
              .map(|behavior| behavior.contains(&Behavior::Completes))
              // behavior is undefined and therefore no subsequent statements shall be executed
              .unwrap_or(false)
          })
          .map(|(statement, _prev)| statement)
          .map(optimize::statement)
          .collect(),
      )
    }

    TypedStatement::IfN1(label, condition, if_body, else_body) => {
      let condition = if statement_behavior(&if_body).is_none() {
        // behavior of `if` branch is undefined and therefore `else` branch shall be taken
        TypedExpression::N1Constant(false)
      } else if else_body
        .as_deref()
        .map(statement_behavior)
        .unwrap_or(Some(HashSet::from([Behavior::Completes])))
        .is_none()
      {
        // behavior of `else` branch is undefined and therefore `if` branch shall be taken
        TypedExpression::N1Constant(true)
      } else {
        condition
      };

      match optimize::expression(condition) {
        TypedExpression::N1Constant(true) => optimize::statement(*if_body),
        TypedExpression::N1Constant(false) => else_body
          .map(|else_body| optimize::statement(*else_body))
          .unwrap_or(TypedStatement::Compound(vec![])),
        condition => TypedStatement::IfN1(
          label,
          condition,
          Box::new(optimize::statement(*if_body)),
          else_body.map(|else_body| Box::new(optimize::statement(*else_body))),
        ),
      }
    }

    TypedStatement::WhileN1(label, condition, body, is_do_while) => {
      let condition = if statement_behavior(&body).is_none() {
        // behavior of `body` is undefined and therefore loop shall not be entered
        TypedExpression::N1Constant(false)
      } else {
        condition
      };

      match (is_do_while, optimize::expression(condition)) {
        (false, TypedExpression::N1Constant(false)) => TypedStatement::Compound(vec![]),
        // `do stmt while (0)` is not equivalent to `stmt;` because `stmt` may contain `break`
        // (true, TypedExpression::N1Constant(false)) => optimize::statement(*body),
        (is_do_while, condition) => TypedStatement::WhileN1(
          label,
          condition,
          Box::new(optimize::statement(*body)),
          is_do_while,
        ),
      }
    }

    TypedStatement::Break(label, locals_size) => TypedStatement::Break(label, locals_size),

    TypedStatement::Continue(label, locals_size) => TypedStatement::Continue(label, locals_size),

    TypedStatement::MacroReturnN0(parameters_size, locals_size, expression) => {
      TypedStatement::MacroReturnN0(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::MacroReturnN1(parameters_size, locals_size, expression) => {
      TypedStatement::MacroReturnN1(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::MacroReturnN8(parameters_size, locals_size, expression) => {
      TypedStatement::MacroReturnN8(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::FunctionReturnN0(parameters_size, locals_size, expression) => {
      TypedStatement::FunctionReturnN0(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::FunctionReturnN1(parameters_size, locals_size, expression) => {
      TypedStatement::FunctionReturnN1(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::FunctionReturnN8(parameters_size, locals_size, expression) => {
      TypedStatement::FunctionReturnN8(
        parameters_size,
        locals_size,
        expression.map(optimize::expression),
      )
    }

    TypedStatement::InitLocalN0(expression) => {
      TypedStatement::InitLocalN0(expression.map(optimize::expression))
    }

    TypedStatement::InitLocalN1(expression) => {
      TypedStatement::InitLocalN1(expression.map(optimize::expression))
    }

    TypedStatement::InitLocalN8(expression) => {
      TypedStatement::InitLocalN8(expression.map(optimize::expression))
    }

    TypedStatement::UninitLocalN0 => TypedStatement::UninitLocalN0,

    TypedStatement::UninitLocalN1 => TypedStatement::UninitLocalN1,

    TypedStatement::UninitLocalN8 => TypedStatement::UninitLocalN8,

    TypedStatement::Assembly(assembly) => TypedStatement::Assembly(assembly),
  }
}

fn expression(expression: TypedExpression) -> TypedExpression {
  // moves comma operators outward. that is, moves operations on a comma expression
  // inside the comma expression. facilitates the extraction of the left-hand side
  // of comma expressions into statements
  macro_rules! default {
    ($expression:expr, $second_variant:ident, $outer_variant:ident) => {
      match $expression {
        TypedExpression::N0SecondN0N0(expression1, expression2)
        | TypedExpression::N1SecondN0N1(expression1, expression2)
        | TypedExpression::N8SecondN0N8(expression1, expression2) => {
          optimize::expression(TypedExpression::$second_variant(
            expression1,
            Box::new(TypedExpression::$outer_variant(expression2)),
          ))
        }
        expression => TypedExpression::$outer_variant(Box::new(expression)),
      }
    };

    ($expression1:expr, $expression2:expr, $second_variant:ident, $outer_variant:ident) => {
      match ($expression1, $expression2) {
        (TypedExpression::N0SecondN0N0(expression1, expression2), expression3)
        | (TypedExpression::N1SecondN0N1(expression1, expression2), expression3)
        | (TypedExpression::N8SecondN0N8(expression1, expression2), expression3) => {
          optimize::expression(TypedExpression::$second_variant(
            expression1,
            Box::new(TypedExpression::$outer_variant(
              expression2,
              Box::new(expression3),
            )),
          ))
        }
        (expression1, TypedExpression::N0SecondN0N0(expression2, expression3))
        | (expression1, TypedExpression::N1SecondN0N1(expression2, expression3))
        | (expression1, TypedExpression::N8SecondN0N8(expression2, expression3)) => {
          optimize::expression(TypedExpression::$second_variant(
            expression2,
            Box::new(TypedExpression::$outer_variant(
              Box::new(expression1),
              expression3,
            )),
          ))
        }
        (expression1, expression2) => {
          TypedExpression::$outer_variant(Box::new(expression1), Box::new(expression2))
        }
      }
    };
  }

  if expression_behavior(&expression).is_none() {
    // behavior is undefined and therefore expression shall not be evaluated
    return TypedExpression::N0Constant(());
  }

  match expression {
    TypedExpression::N1DereferenceN8(expression) => match optimize::expression(*expression) {
      TypedExpression::N8Constant(0x00) => {
        TypedExpression::N1Constant(false) // null pointer dereference. behavior is undefined
      }
      expression => default!(expression, N1SecondN0N1, N1DereferenceN8),
    },

    TypedExpression::N8DereferenceN8(expression) => match optimize::expression(*expression) {
      TypedExpression::N8Constant(0x00) => {
        TypedExpression::N8Constant(0x00) // null pointer dereference. behavior is undefined
      }
      expression => default!(expression, N8SecondN0N8, N8DereferenceN8),
    },

    TypedExpression::N1BitwiseComplement(expression) => match optimize::expression(*expression) {
      TypedExpression::N1BitwiseComplement(expression) => *expression,
      TypedExpression::N1Constant(constant) => TypedExpression::N1Constant(!constant),
      expression => default!(expression, N1SecondN0N1, N1BitwiseComplement),
    },

    TypedExpression::N8BitwiseComplement(expression) => match optimize::expression(*expression) {
      TypedExpression::N8BitwiseComplement(expression) => *expression,
      TypedExpression::N8Constant(constant) => TypedExpression::N8Constant(!constant),
      expression => default!(expression, N8SecondN0N8, N8BitwiseComplement),
    },

    TypedExpression::N8Addition(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_add(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Addition)
        }
      }
    }

    TypedExpression::N8Subtraction(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_sub(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Subtraction)
        }
      }
    }

    TypedExpression::N8Multiplication(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (expression, TypedExpression::N8Constant(0x00))
        | (TypedExpression::N8Constant(0x00), expression) => {
          optimize::expression(TypedExpression::N0SecondN0N0(
            Box::new(TypedExpression::N0CastN8(Box::new(expression))),
            Box::new(TypedExpression::N8Constant(0x00)),
          ))
        }
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_mul(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, N8Multiplication)
        }
      }
    }

    TypedExpression::U8Division(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (_expression, TypedExpression::N8Constant(0x00)) => {
          TypedExpression::N8Constant(0x00) // division by zero. behavior is undefined
        }
        (TypedExpression::N8Constant(0x00), expression) => {
          optimize::expression(TypedExpression::N0SecondN0N0(
            Box::new(TypedExpression::N0CastN8(Box::new(expression))),
            Box::new(TypedExpression::N8Constant(0x00)),
          ))
        }
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_div(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, U8Division)
        }
      }
    }

    TypedExpression::U8Modulo(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (_expression, TypedExpression::N8Constant(0x00)) => {
          TypedExpression::N8Constant(0x00) // modulo zero. behavior is undefined
        }
        (TypedExpression::N8Constant(0x00), expression) => {
          optimize::expression(TypedExpression::N0SecondN0N0(
            Box::new(TypedExpression::N0CastN8(Box::new(expression))),
            Box::new(TypedExpression::N8Constant(0x00)),
          ))
        }
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N8Constant(constant1.wrapping_rem(constant2))
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N8SecondN0N8, U8Modulo)
        }
      }
    }

    TypedExpression::N1EqualToN8(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant(constant1 == constant2)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1EqualToN8)
        }
      }
    }

    TypedExpression::N1LessThanU8(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant(constant1 < constant2)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1LessThanU8)
        }
      }
    }

    TypedExpression::N1LessThanI8(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N8Constant(constant1), TypedExpression::N8Constant(constant2)) => {
          TypedExpression::N1Constant((constant1 as i8) < constant2 as i8)
        }
        (expression1, expression2) => {
          default!(expression1, expression2, N1SecondN0N1, N1LessThanI8)
        }
      }
    }

    TypedExpression::N0SecondN0N0(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N0Constant(_constant), expression) => expression,
        (expression1, expression2) => {
          // `default!` overflows stack when folding `(a, (b, c))`
          TypedExpression::N0SecondN0N0(Box::new(expression1), Box::new(expression2))
        }
      }
    }

    TypedExpression::N1SecondN0N1(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N0Constant(_constant), expression) => expression,
        (expression1, expression2) => {
          // `default!` overflows stack when folding `(a, (b, c))`
          TypedExpression::N1SecondN0N1(Box::new(expression1), Box::new(expression2))
        }
      }
    }

    TypedExpression::N8SecondN0N8(expression1, expression2) => {
      match (
        optimize::expression(*expression1),
        optimize::expression(*expression2),
      ) {
        (TypedExpression::N0Constant(_constant), expression) => expression,
        (expression1, expression2) => {
          // `default!` overflows stack when folding `(a, (b, c))`
          TypedExpression::N8SecondN0N8(Box::new(expression1), Box::new(expression2))
        }
      }
    }

    // move bitwise truncations inward. that is, turn a truncation of the result of an
    // operation into a simpler operation on truncated operands. optimizes away null
    // statements that have no side effects, for example
    //
    TypedExpression::N0CastN1(expression) => match optimize::expression(*expression) {
      TypedExpression::N1DereferenceN8(_) => TypedExpression::N0Constant(()),
      TypedExpression::N1BitwiseComplement(expression) => {
        optimize::expression(TypedExpression::N0CastN8(expression))
      }
      TypedExpression::N1EqualToN8(expression1, expression2)
      | TypedExpression::N1LessThanU8(expression1, expression2)
      | TypedExpression::N1LessThanI8(expression1, expression2)
      | TypedExpression::U8Division(expression1, expression2)
      | TypedExpression::U8Modulo(expression1, expression2) => {
        optimize::expression(TypedExpression::N0SecondN0N0(
          Box::new(TypedExpression::N0CastN8(expression1)),
          Box::new(TypedExpression::N0CastN8(expression2)),
        ))
      }
      TypedExpression::N1CastN8(expression) => {
        optimize::expression(TypedExpression::N0CastN8(expression))
      }
      TypedExpression::N1Constant(_constant) => TypedExpression::N0Constant(()),
      expression => default!(expression, N0SecondN0N0, N0CastN1),
    },

    TypedExpression::N0CastN8(expression) => match optimize::expression(*expression) {
      TypedExpression::N8DereferenceN8(_) => TypedExpression::N0Constant(()),
      TypedExpression::N8BitwiseComplement(expression) => {
        optimize::expression(TypedExpression::N0CastN8(expression))
      }
      TypedExpression::N8Addition(expression1, expression2)
      | TypedExpression::N8Subtraction(expression1, expression2)
      | TypedExpression::N8Multiplication(expression1, expression2)
      | TypedExpression::U8Division(expression1, expression2)
      | TypedExpression::U8Modulo(expression1, expression2) => {
        optimize::expression(TypedExpression::N0SecondN0N0(
          Box::new(TypedExpression::N0CastN8(expression1)),
          Box::new(TypedExpression::N0CastN8(expression2)),
        ))
      }
      TypedExpression::N8Constant(_)
      | TypedExpression::N8LoadLocal(_)
      | TypedExpression::N8AddrLocal(_)
      | TypedExpression::N8LoadGlobal(_)
      | TypedExpression::N8AddrGlobal(_) => TypedExpression::N0Constant(()),
      expression => default!(expression, N0SecondN0N0, N0CastN8),
    },

    TypedExpression::N1CastN8(expression) => match optimize::expression(*expression) {
      TypedExpression::N8BitwiseComplement(expression) => optimize::expression(
        TypedExpression::N1BitwiseComplement(Box::new(TypedExpression::N1CastN8(expression))),
      ),
      TypedExpression::N8Addition(_expression1, _expression2)
      | TypedExpression::N8Subtraction(_expression1, _expression2)
      | TypedExpression::N8Multiplication(_expression1, _expression2) => {
        todo!()
        // N8Addition => psi N1Addition N1CastN8
        // N8Subtraction => psi N1Subtraction N1CastN8
        // N8Multiplication => psi N1Multiplication N1CastN8
      }
      TypedExpression::N8Constant(constant) => {
        TypedExpression::N1Constant((constant & 0x01) != 0x00)
      }
      expression => default!(expression, N1SecondN0N1, N1CastN8),
    },

    TypedExpression::N0Constant(constant) => TypedExpression::N0Constant(constant),

    TypedExpression::N1Constant(constant) => TypedExpression::N1Constant(constant),

    TypedExpression::N8Constant(constant) => TypedExpression::N8Constant(constant),

    TypedExpression::N8LoadLocal(offset) => TypedExpression::N8LoadLocal(offset),

    TypedExpression::N8AddrLocal(offset) => TypedExpression::N8AddrLocal(offset),

    TypedExpression::N8LoadGlobal(label) => TypedExpression::N8LoadGlobal(label),

    TypedExpression::N8AddrGlobal(label) => TypedExpression::N8AddrGlobal(label),

    TypedExpression::N0MacroCall(label, arguments) => TypedExpression::N0MacroCall(
      label,
      arguments.into_iter().map(optimize::expression).collect(),
    ),

    TypedExpression::N1MacroCall(label, arguments) => TypedExpression::N1MacroCall(
      label,
      arguments.into_iter().map(optimize::expression).collect(),
    ),

    TypedExpression::N8MacroCall(label, arguments) => TypedExpression::N8MacroCall(
      label,
      arguments.into_iter().map(optimize::expression).collect(),
    ),

    TypedExpression::N0FunctionCall(designator, arguments) => TypedExpression::N0FunctionCall(
      Box::new(optimize::expression(*designator)),
      arguments.into_iter().map(optimize::expression).collect(),
    ),

    TypedExpression::N1FunctionCall(designator, arguments) => TypedExpression::N1FunctionCall(
      Box::new(optimize::expression(*designator)),
      arguments.into_iter().map(optimize::expression).collect(),
    ),

    TypedExpression::N8FunctionCall(designator, arguments) => TypedExpression::N8FunctionCall(
      Box::new(optimize::expression(*designator)),
      arguments.into_iter().map(optimize::expression).collect(),
    ),
  }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Behavior {
  Breaks(String),    // control flow breaks out of a loop
  Continues(String), // control flow continues to the next iteration of a loop
  Returns,           // control flow returns from a function
  Hangs,             // control flow hangs indefinitely
  Completes,         // control flow completes normally
}

fn behavior_alternation(
  a: Option<HashSet<Behavior>>,
  b: Option<HashSet<Behavior>>,
) -> Option<HashSet<Behavior>> {
  // either code path may be taken, as with an `if` statement

  match (a, b) {
    (Some(a), Some(b)) => Some(a.into_iter().chain(b).collect()),
    (Some(a), None) => Some(a),
    (None, Some(b)) => Some(b),
    (None, None) => None,
  }
}

fn behavior_unsequenced(
  a: Option<HashSet<Behavior>>,
  b: Option<HashSet<Behavior>>,
) -> Option<HashSet<Behavior>> {
  // both paths are taken but unsequenced, as with a `+` operator

  match (a, b) {
    (Some(a), Some(b)) => Some(a.into_iter().chain(b).collect()),
    (Some(_), None) => None,
    (None, Some(_)) => None,
    (None, None) => None,
  }
}

fn behavior_sequenced(
  a: Option<HashSet<Behavior>>,
  b: Option<HashSet<Behavior>>,
) -> Option<HashSet<Behavior>> {
  // both paths are taken and sequenced, as with a `,` operator or compound statement

  match (a, b) {
    (Some(a), Some(b)) => match a.contains(&Behavior::Completes) {
      true => Some(
        a.into_iter()
          .filter(|x| *x != Behavior::Completes)
          .chain(b)
          .collect(),
      ),
      false => Some(a),
    },
    (Some(_), None) => None,
    (None, Some(_)) => None,
    (None, None) => None,
  }
}

fn behavior_difference(
  a: Option<HashSet<Behavior>>,
  b: HashSet<Behavior>,
) -> Option<HashSet<Behavior>> {
  // remove from possible behaviors, leaving undefined behavior untouched

  match a {
    Some(a) => Some(a.into_iter().filter(|x| !b.contains(x)).collect()),
    None => None,
  }
}

fn behavior_union(a: Option<HashSet<Behavior>>, b: HashSet<Behavior>) -> Option<HashSet<Behavior>> {
  // add to possible behaviors, leaving undefined behavior untouched

  match a {
    Some(a) => Some(a.into_iter().chain(b).collect()),
    None => None,
  }
}

pub fn behavior_contains(a: &Option<HashSet<Behavior>>, b: &Behavior) -> bool {
  match a {
    Some(a) => a.contains(b),
    None => false,
  }
}

pub fn statement_behavior(statement: &TypedStatement) -> Option<HashSet<Behavior>> {
  // determine the set of possible runtime behaviors of a statement. the output is a
  // superset of the actual runtime behavior. a `None` indicates the behavior is undefined.

  match statement {
    TypedStatement::ExpressionN0(expression) => expression_behavior(expression),

    TypedStatement::Compound(statements) => statements.iter().map(statement_behavior).fold(
      Some(HashSet::from([Behavior::Completes])),
      behavior_sequenced,
    ),

    TypedStatement::IfN1(_label, condition, if_body, else_body) => {
      let if_body_behavior = statement_behavior(if_body);
      let else_body_behavior = else_body
        .as_deref()
        .map(statement_behavior)
        .unwrap_or(Some(HashSet::from([Behavior::Completes])));

      behavior_sequenced(
        expression_behavior(condition),
        match optimize::expression(condition.clone()) {
          TypedExpression::N1Constant(true) => if_body_behavior,
          TypedExpression::N1Constant(false) => else_body_behavior,
          _ => behavior_alternation(if_body_behavior, else_body_behavior),
        },
      )
    }

    TypedStatement::WhileN1(label, condition, body, is_do_while) => {
      let body_behavior = statement_behavior(body);
      let body_behavior =
        match behavior_contains(&body_behavior, &Behavior::Continues(label.clone())) {
          true => behavior_union(body_behavior, HashSet::from([Behavior::Hangs])),
          false => body_behavior,
        };
      let body_behavior = match behavior_contains(&body_behavior, &Behavior::Breaks(label.clone()))
      {
        true => behavior_union(body_behavior, HashSet::from([Behavior::Completes])),
        false => body_behavior,
      };

      let loop_behavior = behavior_difference(
        match (is_do_while, optimize::expression(condition.clone())) {
          (false, TypedExpression::N1Constant(false)) => Some(HashSet::from([Behavior::Completes])),
          (true, TypedExpression::N1Constant(false)) => body_behavior,
          (_is_do_while, TypedExpression::N1Constant(true)) => {
            match behavior_contains(&body_behavior, &Behavior::Breaks(label.clone())) {
              true => behavior_union(body_behavior, HashSet::from([Behavior::Hangs])),
              false => behavior_difference(
                behavior_union(body_behavior, HashSet::from([Behavior::Hangs])),
                HashSet::from([Behavior::Completes]),
              ),
            }
          }
          (false, _condition) => behavior_alternation(
            behavior_union(body_behavior, HashSet::from([Behavior::Hangs])),
            Some(HashSet::from([Behavior::Completes])),
          ),
          (true, _condition) => behavior_union(body_behavior, HashSet::from([Behavior::Hangs])),
        },
        HashSet::from([
          Behavior::Breaks(label.clone()),
          Behavior::Continues(label.clone()),
        ]),
      );

      match is_do_while {
        false => behavior_sequenced(expression_behavior(condition), loop_behavior),
        true => behavior_sequenced(loop_behavior, expression_behavior(condition)),
      }
    }

    TypedStatement::Break(label, _locals_size) => {
      Some(HashSet::from([Behavior::Breaks(label.clone())]))
    }

    TypedStatement::Continue(label, _locals_size) => {
      Some(HashSet::from([Behavior::Continues(label.clone())]))
    }

    TypedStatement::MacroReturnN0(_, _, _)
    | TypedStatement::MacroReturnN1(_, _, _)
    | TypedStatement::MacroReturnN8(_, _, _)
    | TypedStatement::FunctionReturnN0(_, _, _)
    | TypedStatement::FunctionReturnN1(_, _, _)
    | TypedStatement::FunctionReturnN8(_, _, _) => Some(HashSet::from([Behavior::Returns])),

    TypedStatement::InitLocalN0(expression)
    | TypedStatement::InitLocalN1(expression)
    | TypedStatement::InitLocalN8(expression) => expression
      .as_ref()
      .map(expression_behavior)
      .unwrap_or(Some(HashSet::from([Behavior::Completes]))),

    TypedStatement::UninitLocalN0
    | TypedStatement::UninitLocalN1
    | TypedStatement::UninitLocalN8 => Some(HashSet::from([Behavior::Completes])),

    TypedStatement::Assembly(_assembly) => Some(HashSet::from([
      Behavior::Returns,
      Behavior::Hangs,
      Behavior::Completes,
    ])),
  }
}

pub fn expression_behavior(expression: &TypedExpression) -> Option<HashSet<Behavior>> {
  match expression {
    TypedExpression::N1DereferenceN8(expression) | TypedExpression::N8DereferenceN8(expression)
      if matches!(**expression, TypedExpression::N8Constant(0x00)) =>
    {
      None // null pointer dereference. behavior is undefined
    }

    TypedExpression::N1DereferenceN8(expression)
    | TypedExpression::N8DereferenceN8(expression)
    | TypedExpression::N1BitwiseComplement(expression)
    | TypedExpression::N8BitwiseComplement(expression) => expression_behavior(expression),

    TypedExpression::U8Division(_expression1, expression2)
    | TypedExpression::U8Modulo(_expression1, expression2)
      if matches!(**expression2, TypedExpression::N8Constant(0x00)) =>
    {
      None // division by zero. behavior is undefined
    }

    TypedExpression::N8Addition(expression1, expression2)
    | TypedExpression::N8Subtraction(expression1, expression2)
    | TypedExpression::N8Multiplication(expression1, expression2)
    | TypedExpression::U8Division(expression1, expression2)
    | TypedExpression::U8Modulo(expression1, expression2) => behavior_unsequenced(
      expression_behavior(expression1),
      expression_behavior(expression2),
    ),

    TypedExpression::N1EqualToN8(expression1, expression2)
    | TypedExpression::N1LessThanU8(expression1, expression2)
    | TypedExpression::N1LessThanI8(expression1, expression2) => behavior_unsequenced(
      expression_behavior(expression1),
      expression_behavior(expression2),
    ),

    // using `behavior_sequence` because there is a sequence point between the two operands
    TypedExpression::N0SecondN0N0(expression1, expression2)
    | TypedExpression::N1SecondN0N1(expression1, expression2)
    | TypedExpression::N8SecondN0N8(expression1, expression2) => behavior_sequenced(
      expression_behavior(expression1),
      expression_behavior(expression2),
    ),

    TypedExpression::N0CastN1(expression)
    | TypedExpression::N0CastN8(expression)
    | TypedExpression::N1CastN8(expression) => expression_behavior(expression),

    TypedExpression::N0Constant(_)
    | TypedExpression::N1Constant(_)
    | TypedExpression::N8Constant(_)
    | TypedExpression::N8LoadLocal(_)
    | TypedExpression::N8AddrLocal(_)
    | TypedExpression::N8LoadGlobal(_)
    | TypedExpression::N8AddrGlobal(_) => Some(HashSet::from([Behavior::Completes])),

    TypedExpression::N0MacroCall(_label, arguments)
    | TypedExpression::N1MacroCall(_label, arguments)
    | TypedExpression::N8MacroCall(_label, arguments) => {
      // using `behavior_sequence` because there is a sequence point before the call
      behavior_sequenced(
        // the order of evaluation of the arguments is unspecified
        arguments.iter().map(expression_behavior).fold(
          Some(HashSet::from([Behavior::Completes])),
          behavior_unsequenced,
        ),
        // the calle itself may hang or complete
        Some(HashSet::from([Behavior::Hangs, Behavior::Completes])),
      )
    }

    TypedExpression::N0FunctionCall(designator, arguments)
    | TypedExpression::N1FunctionCall(designator, arguments)
    | TypedExpression::N8FunctionCall(designator, arguments) => {
      // using `behavior_sequence` because there is a sequence point before the call
      behavior_sequenced(
        // the order of evaluation of the designator and arguments is unspecified
        behavior_unsequenced(
          expression_behavior(designator),
          arguments.iter().map(expression_behavior).fold(
            Some(HashSet::from([Behavior::Completes])),
            behavior_unsequenced,
          ),
        ),
        // the calle itself may hang or complete
        Some(HashSet::from([Behavior::Hangs, Behavior::Completes])),
      )
    }
  }
}
