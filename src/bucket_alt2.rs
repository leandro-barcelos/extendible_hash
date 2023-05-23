use std::{fmt::Display, fs::File, io::Read};

#[derive(Clone)]
pub struct BucketAlt2 {
    pub name: String,
    pub local_depth: u8,
    pub data: Vec<((String, i32), (usize, usize))>,
    pub size: u8,
}

impl BucketAlt2 {
    pub fn new(name: String, local_depth: u8, size: u8) -> Self {
        BucketAlt2 {
            name,
            local_depth,
            data: Vec::with_capacity(size as usize),
            size,
        }
    }

    pub fn insert(&mut self, d: ((String, i32), (usize, usize))) -> bool {
        if self.data.len() == self.size as usize {
            return false;
        }

        self.data.push(d);

        true
    }

    pub fn remove(&mut self, key: (String, i32)) -> bool {
        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                self.data.remove(i);
                return true;
            }
        }

        false
    }

    pub fn serialize(&self) -> Vec<u8> {
        // | 3B name | 1B ld | 1B size | R1 104B | R2 104B |...|Rsize 104B |

        let mut encoded: Vec<u8> = Vec::with_capacity(5 + self.size as usize * 100);

        // Name
        let name_as_byte = self.name.as_bytes();

        for i in 0..3 {
            if let Some(c) = name_as_byte.get(i) {
                encoded.push(*c);
            } else {
                encoded.push(0);
            }
        }

        // Local Depth
        encoded.push(self.local_depth.to_be_bytes()[0]);

        // Size
        encoded.push(self.size.to_be_bytes()[0]);

        let n = encoded.len();

        // Data
        for _ in 0..(104 * self.size as usize) {
            encoded.push(0);
        }

        for (i, record) in self.data.iter().enumerate() {
            let mut start = n + (104 * i);

            // Key
            // text
            let tmp = record.0 .0.as_bytes();

            for byte in tmp {
                encoded[start] = *byte;
                start += 1;
            }
            //nseq
            let tmp = record.0 .1.to_be_bytes();

            for byte in tmp {
                encoded[start] = byte;
                start += 1;
            }
            // Rid
            // PageId
            let tmp = (record.1 .0 as u16).to_be_bytes();

            for byte in tmp {
                encoded[start] = byte;
                start += 1;
            }
            // SlotId
            let tmp = (record.1 .1 as u16).to_be_bytes();

            for byte in tmp {
                encoded[start] = byte;
                start += 1;
            }
        }

        encoded
    }

    pub fn deserialize(f: &mut File) -> Self {
        // Name
        let mut buffer = [0; 3];

        f.read(&mut buffer).unwrap();
        let name = String::from_utf8(buffer.to_vec()).unwrap();

        // Local Depth
        let mut buffer = [0; 1];

        f.read(&mut buffer).unwrap();

        let local_depth = u8::from_be_bytes(buffer);

        // Bucket size
        f.read(&mut buffer).unwrap();

        let size: usize = u8::from_be_bytes(buffer) as usize;

        // Data
        let mut data: Vec<((String, i32), (usize, usize))> = Vec::new();

        for _ in 0..size {
            //Key
            let tmp_key: (String, i32);

            // Text
            let text: String;
            let mut buffer = [0; 96];

            f.read(&mut buffer).unwrap();

            text = String::from_utf8(buffer.to_vec()).unwrap();
            let text = text.trim_matches('\0').to_string();

            // Nseq
            let mut buffer = [0; 4];
            let nseq: i32;

            f.read(&mut buffer).unwrap();

            nseq = i32::from_be_bytes(buffer);

            tmp_key = (text, nseq);

            // Rid
            let rid: (usize, usize);
            // PageID
            let mut buffer = [0; 2];
            let pageid: usize;

            f.read(&mut buffer).unwrap();

            pageid = u16::from_be_bytes(buffer) as usize;

            // SlotID
            let mut buffer = [0; 2];
            let slotid: usize;

            f.read(&mut buffer).unwrap();

            slotid = u16::from_be_bytes(buffer) as usize;

            rid = (pageid, slotid);

            if !tmp_key.0.is_empty() {
                data.push((tmp_key, rid));
            }
        }

        BucketAlt2 {
            name,
            local_depth,
            data,
            size: size as u8,
        }
    }
}

impl Display for BucketAlt2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        // Local depth
        let depth_square = format!("+---+\n|{: ^3}|\n", self.local_depth);
        s.push_str(format!("{}", depth_square).as_str());

        // Bucket
        let sep = format!("+{}", "---+".to_string().repeat(self.size as usize));
        s.push_str(format!("{sep}\n|").as_str());

        for i in 0..self.size as usize {
            if let Some(d) = self.data.get(i) {
                s.push_str(format!("{: ^3}|", d.0 .1).as_str())
            } else {
                s.push_str(format!("   |",).as_str())
            }
        }

        s.push_str(format!(" {}", self.name).as_str());

        s.push_str(format!("\n{}", sep.as_str()).as_str());

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_bucket_display_size_4() {
        let b = BucketAlt2::new("A".to_string(), 2, 4);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    #[test]
    fn test_bucket_display_size_8() {
        let b = BucketAlt2::new("A".to_string(), 2, 8);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    #[test]
    fn test_bucket_display_size_16() {
        let b = BucketAlt2::new("A".to_string(), 2, 16);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    // #[test]
    // fn test_serialize() {
    //     let b1 = BucketAlt2 {
    //         name: "A".to_string(),
    //         local_depth: 2,
    //         data: vec![
    //             (Record{nseq: 0, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()}),
    //             (Record{nseq: 1, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()}),
    //             (Record{nseq: 2, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()}),
    //             (Record{nseq: 3, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()}),
    //         ],
    //         size: 4,
    //     };

    //     // let mut b2 = b1.clone();
    //     // b2.data.pop();

    //     let encoded1 = b1.serialize();
    //     // let encoded2 = b2.serialize();

    //     // for byte in encoded1 {
    //     //     print!("{:08b} ", byte);
    //     // }

    //     let mut file1 = File::create("b1.bin").unwrap();

    //     file1.write_all(&encoded1).unwrap();

    //     // for byte in encoded2 {
    //     //     print!("{:08b} ", byte);
    //     // }

    //     assert_eq!("", "")
    // }
}
