

pub struct Identifier {
    namespace: String,
    path: String,
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

