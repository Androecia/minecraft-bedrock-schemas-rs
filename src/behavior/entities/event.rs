use crate::general::identifier::Identifier;

pub struct Event {
    identifier: Identifier,
}
impl Event {
  pub fn get_identifier(&self) -> Identifier {
      self.identifier
  }
}
