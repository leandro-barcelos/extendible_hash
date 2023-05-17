use std::{fmt::Display, fs::File, io::Read};

#[derive(Clone, PartialEq, Hash)]
pub struct Bucket {
    pub name: String,
    pub local_depth: u8,
    pub data: Vec<(i32, String)>,
    pub size: u8,
}

impl Bucket {
    pub fn new(name: String, local_depth: u8, size: u8) -> Self {
        Bucket {
            name,
            local_depth,
            data: Vec::with_capacity(size as usize),
            size,
        }
    }

    pub fn insert(&mut self, record: (i32, String)) -> bool {
        if self.data.len() == self.size as usize {
            return false;
        }

        self.data.push(record);

        true
    }

    pub fn remove(&mut self, key: i32) -> bool {
        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                self.data.remove(i);
                return true;
            }
        }

        false
    }

    pub fn search(&self, key: i32) -> Option<(i32, String)> {
        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                return Some(self.data[i].clone());
            }
        }

        None
    }

    pub fn serialize(&self) -> Vec<u8> {
        // | 3B name | 1B ld | 1B size | R1 100B | R2 100B |...|Rsize 100B |

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

        for _ in 0..(100 * self.size as usize) {
            encoded.push(0);
        }

        for (i, record) in self.data.iter().enumerate() {
            let mut start = n + (100 * i);
            let tmp = record.0.to_be_bytes();

            for byte in tmp {
                encoded[start] = byte;
                start += 1;
            }

            let tmp = record.1.as_bytes();

            for byte in tmp {
                encoded[start] = *byte;
                start += 1;
            }
        }

        encoded
    }

    pub fn deserialize(f: &mut File) -> Self {
        let mut buffer = [0; 3];

        f.read(&mut buffer).unwrap();
        let mut name = String::new();

        for byte in buffer {
            name.push(char::from_u32(byte as u32).unwrap());
        }

        let mut buffer = [0; 1];

        f.read(&mut buffer).unwrap();

        let local_depth = u8::from_be_bytes(buffer);

        f.read(&mut buffer).unwrap();

        let size: usize = u8::from_be_bytes(buffer) as usize;
        let mut data: Vec<(i32, String)> = Vec::new();

        for _ in 0..size {
            let mut buffer = [0; 4];
            let nseq: i32;
            let text: String;

            f.read(&mut buffer).unwrap();

            nseq = i32::from_be_bytes(buffer);

            let mut buffer = [0; 96];

            f.read(&mut buffer).unwrap();

            text = String::from_utf8(buffer.to_vec()).unwrap();

            data.push((nseq, text));
        }

        Bucket {
            name,
            local_depth,
            data,
            size: size as u8,
        }
    }
}

impl Display for Bucket {
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
                s.push_str(format!("{: ^3}|", d.0).as_str())
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
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn test_bucket_display_size_4() {
        let b = Bucket::new("A".to_string(), 2, 4);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    #[test]
    fn test_bucket_display_size_8() {
        let b = Bucket::new("A".to_string(), 2, 8);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    #[test]
    fn test_bucket_display_size_16() {
        let b = Bucket::new("A".to_string(), 2, 16);

        println!("{b}");

        assert_eq!(format!("{b}"), "_");
    }

    #[test]
    fn test_serialize() {
        let b1 = Bucket {
            name: "A".to_string(),
            local_depth: 2,
            data: vec![
                (0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()),
                (1, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()),
                (2, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()),
                (3, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()),
            ],
            size: 4,
        };

        // let mut b2 = b1.clone();
        // b2.data.pop();

        let encoded1 = b1.serialize();
        // let encoded2 = b2.serialize();

        // for byte in encoded1 {
        //     print!("{:08b} ", byte);
        // }

        let mut file1 = File::create("b1.bin").unwrap();

        file1.write_all(&encoded1).unwrap();

        // for byte in encoded2 {
        //     print!("{:08b} ", byte);
        // }

        assert_eq!("", "")
    }
}
