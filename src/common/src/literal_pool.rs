#![allow(dead_code)]

use std::collections::HashMap;

use crate::{literal::Literal, shape::Shape};

// Literal pool provides a mechanism to deduplicate identical literals and
// share them across multiple HLO modules.
pub struct LiteralPool {
  literals: HashMap<Shape, Vec<Literal>>
}

impl LiteralPool {
  // Returns a default literal pool that can be used across multiple HLO modules
  // in a process.
  pub fn default() -> Self {
    LiteralPool { literals: HashMap::new() }
  }

  // Returns a canonical literal from the pool. If the literal is not in the
  // pool, it is added to the pool and returned back.
  pub fn get_canonical_literal(&self, literal: &Literal) -> Option<&Literal> {
    let literals: Option<&Vec<Literal>> = self.literals.get(literal.shape());
    self.find_canonical_literal(literals.unwrap(), literal)
  }

  pub fn create_and_set_canonical_literal(&mut self, literal: &Literal) -> Option<&Literal> {
    let mut new_literal = Literal::default();
    let result = new_literal.copy_from(
      literal, &vec![], &vec![], false);
    assert!(result.is_ok());

    let mut literals = self.literals.get_mut(literal.shape());
    assert!(literals.is_some());
    literals.as_mut().unwrap().push(new_literal);
    literals.unwrap().last()
  }

  // Runs garbage collection on all the literals in the pool. Returns the number
  // of literals that were garbage collected.
  pub fn garbage_collect(&self) -> usize {
    let mut num_erased = 0;
    for (_s, literals) in &self.literals {
      num_erased += self.erase_expired_literals(literals);
    }
    num_erased
  }

  // Runs garbage collection on literals with the given shape. Returns the
  // number of literals that were garbage collected.
  pub fn garbage_collect_by_shape(&self, shape: &Shape) -> usize {
    let literals: Option<&Vec<Literal>> = self.literals.get(shape);
    if literals.is_none() { return 0; }
    let num_erased = self.erase_expired_literals(literals.unwrap());

    println!("Garbage collected {:?} literals for shape {:?}",
      num_erased, shape.to_string(false));
    num_erased
  }

  // Erases expired weak pointers from the vector and returns the number of
  // elements that were erased.
  fn erase_expired_literals(&self, _literals: &Vec<Literal>) -> usize {
    unimplemented!()
  }

  // Tried to find a canonical literal in the pool. Return nullptr if not found.
  fn find_canonical_literal<'a>(
    &self, literals: &'a Vec<Literal>, literal: &Literal) -> Option<&'a Literal>
  {
    for l in literals {
      if l.equal(literal, false) {
        return Some(l);
      }
    }
    None
  }
}