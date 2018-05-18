#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Keyword {
    Return,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}
