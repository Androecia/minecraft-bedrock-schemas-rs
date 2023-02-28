use crate::general::identifier::NamespaceIdentifier;

pub struct Event {
    identifier: NamespaceIdentifier,
}
impl Event {
  pub fn get_identifier(&self) -> &NamespaceIdentifier {
      &self.identifier
  }
}
