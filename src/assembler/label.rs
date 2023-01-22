#[allow(dead_code)]
pub enum LabelType {
    None,
    Data,
    Text
}

#[allow(dead_code)]
pub struct Label {
    pub name: String,
    pub l_type: LabelType,
    pub addr: u16,
    pub value: String
}

impl Label {
    pub fn new() -> Label {
        Label {
            name: String::from(""),
            l_type: LabelType::None,
            addr: 0,
            value: String::from("")
        }
    }

    pub fn create_label(name: String, l_type: LabelType, line_num: u16) -> Label {
        Label {
            name: name,
            l_type: l_type,
            addr: line_num * 4,
            value: String::from("")
        }
    }
}
