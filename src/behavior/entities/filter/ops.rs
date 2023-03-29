



use serde::{Deserialize, Serialize};
/// The comparison to apply with `value`.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
  /// Test for inequality.
  NotEquals,
  /// Test for less-than the value.
  LessThan,
  /// Test for less-than or equal to the value.
  LessThanOrEquals,
  /// Test for equality.
  Equals,
  /// Test for greater-than the value.
  GreaterThan,
  /// Test for greater-than or equal to the value.
  GreaterThanOrEquals,
}

impl Serialize for Operator {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      Operator::NotEquals => serializer.serialize_str("!="),
      Operator::LessThan => serializer.serialize_str("<"),
      Operator::LessThanOrEquals => serializer.serialize_str("<="),
      Operator::Equals => serializer.serialize_str("="),
      Operator::GreaterThan => serializer.serialize_str(">"),
      Operator::GreaterThanOrEquals => serializer.serialize_str(">="),
    }
  }
}

impl<'de> Deserialize<'de> for Operator {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "!=" | "<>" | "not"=> Ok(Operator::NotEquals),
      "<" => Ok(Operator::LessThan),
      "<=" => Ok(Operator::LessThanOrEquals),
      "=" | "==" | "equals" => Ok(Operator::Equals),
      ">" => Ok(Operator::GreaterThan),
      ">=" => Ok(Operator::GreaterThanOrEquals),
      _ => Err(serde::de::Error::custom(format!(
        "unknown operator: {}",
        s
      ))),
    }
  }
}

impl Default for Operator {
  fn default() -> Self {
    Operator::Equals
  }
}
