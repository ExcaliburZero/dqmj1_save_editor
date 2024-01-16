#[derive(Clone, Debug)]
pub struct Monster {
    name: String,
}

impl Monster {
    pub fn new(name: &str) -> Monster {
        Monster {
            name: name.to_string(),
        }
    }
}
