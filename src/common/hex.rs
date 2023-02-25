pub trait Hex {
    type Output;
    fn to_hex_string(&self) -> Self::Output;
}

impl Hex for u8 {
    type Output = String;

    fn to_hex_string(&self) -> Self::Output {
        format!("{:02X}", self)
    }
}

impl Hex for u16 {
    type Output = String;

    fn to_hex_string(&self) -> Self::Output {
        format!("{:04X}", self)
    }
}

// impl Hex for Vec<u8> {
//     type Output = String;

//     fn to_hex_string(&self, start_index: Option<usize>) -> Self::Output {
//         /*
//               00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
//         0000  53 74 72 69 6E 67 FF 00 00 01 05 03 04 05 1B 0A String..........
//          */
//         let mut string = String::from("      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n");

//         let chunks: Vec<&[u8]> = self.chunks(0x10).collect();

//         let mut i = 0;

//         for chunk in chunks {
//             let mut base = format!("{}  ", (i as u16).to_hex_string());
//             let values_left: u8 = 0x10 - (chunk.len() as u8);

//             for val in chunk {
//                 base += &format!("{} ", val.to_hex_string());
//             }

//             if values_left > 0 {
//                 for _ in 0..values_left {
//                     base += "   ";
//                 }
//             }

//             for ascii_val in chunk {
//                 if ascii_val.is_ascii() && !ascii_val.is_ascii_control() {
//                     base.push(ascii_val.clone() as char);
//                     continue;
//                 }

//                 base.push('.');
//             }

//             base += "\n";
//             string += &base;
//             i += 16;
//         }

//         string
//     }
// }
