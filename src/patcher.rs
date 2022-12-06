use crate::edit::Edit;
use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug)]
pub(crate) struct Patcher {
    edits: Vec::<Edit>
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
  #[error("Invalid line number")]
  LineNumber,
}

impl Patcher {
    pub(crate) fn new(edits: Vec::<Edit>) -> Self {
        Self { edits }
    }

    pub(crate) fn patch(&self, mut lines: Vec::<String>) -> Result<String, Error> {
        let len = lines.len();
        for edit in &self.edits {
            if edit.number > len.try_into().unwrap() {
                return Err(Error::LineNumber);
            }
            let index = usize::try_from(edit.number).unwrap();
            lines[index] = edit.text.clone();
        }
        return Ok(lines.join(""));
    }
}
