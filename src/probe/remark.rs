use crate::internal::{lexer::CueLexer, tokenizer::Tokenizer};

pub struct RemarkIter<'a> {
  lexer: CueLexer<'a>,
}

impl<'a> RemarkIter<'a> {
  pub(super) const fn new(buffer: &'a str) -> Self {
    Self {
      lexer: CueLexer::new(Tokenizer::new(buffer)),
    }
  }
}

impl<'a> Iterator for RemarkIter<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.lexer.next_command() {
        Ok(Some(crate::core::command::Command::Remark { value })) => {
          return Some(value);
        }
        Ok(None) => return None,
        _ => continue,
      }
    }
  }
}
