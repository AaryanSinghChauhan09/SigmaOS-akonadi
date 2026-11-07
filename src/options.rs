//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Book(String), Contact(usize, String, String, String, String), Find(usize, String), Remove(usize, usize), List(usize) }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
