#[derive(Debug, PartialEq, Clone)]
pub enum LabelType {
    None,
    Data,
    Text,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub l_type: LabelType,
    pub addr: u16,
    pub value: String,
}

impl Label {
    pub fn new() -> Label {
        Label {
            name: String::from(""),
            l_type: LabelType::None,
            addr: 0,
            value: String::from(""),
        }
    }

    pub fn create_data(name: String, address_offset: u16, value: String) -> Label {
        Label {
            name,
            l_type: LabelType::Data,
            addr: address_offset + 2,
            value,
        }
    }

    pub fn create_text(name: String, address_offset: u16, text_label_offset: u16) -> Label {
        Label {
            name,
            l_type: LabelType::Text,
            addr: address_offset + text_label_offset + 2,
            value: String::from(""),
        }
    }
}
