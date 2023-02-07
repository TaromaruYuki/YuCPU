use super::LabelType;

#[derive(Debug)]
pub struct Token {
    pub label: String,
    pub instruction: String,
    pub args: Vec<String>,
    pub section: LabelType,
}

impl Token {
    pub fn create_token(
        label: String,
        instruction: String,
        args: Vec<String>,
        section: LabelType,
    ) -> Token {
        Token {
            label,
            instruction,
            args,
            section,
        }
    }
}
