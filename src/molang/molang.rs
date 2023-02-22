use std::str::FromStr;

pub struct Molang {}

//TODO: Implement Molang
impl ToString for Molang {
    fn to_string (&self) -> String {
        String::from("Molang")
    }
}


impl Molang {
    pub fn new () -> Self {
        Self {}
    }
}

impl FromStr for Molang {
    type Err = ();

    fn from_str (s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}


impl From<String > for Molang {
    fn from (s: String) -> Self {
        Self {}
    }
}
