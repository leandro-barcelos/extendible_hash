use crate::{
    bucket_alt1::*,
    random_util::{random_string, unique_random_numbers},
    record::Record,
};
use core::fmt;
use std::{fs::File, io::Read};

pub struct HashAlt1 {
    global_depth: u8,
    directory: Vec<usize>,
    pub buckets: Vec<BucketAlt1>,
}

impl HashAlt1 {
    pub fn new(global_depth: u8, bucket_size: u8) -> Self {
        let size = 2_u32.pow(global_depth as u32) as usize;

        let mut directory = Vec::with_capacity(size);
        let mut buckets = Vec::with_capacity(size);

        let mut init_name = "ZZ".to_string();

        for i in 0..size {
            init_name = next_string(&init_name);
            buckets.push(BucketAlt1::new(
                init_name.clone(),
                global_depth,
                bucket_size,
            ));

            directory.push(i);
        }

        HashAlt1 {
            global_depth,
            directory,
            buckets,
        }
    }

    pub fn hash_fun(&self, num: i32) -> usize {
        (num % 2_i32.pow(self.global_depth as u32)) as usize
    }

    pub fn insert(&mut self, record: Record) -> bool {
        if let Some(_) = self.search(record.nseq) {
            return false;
        }

        let h = self.hash_fun(record.nseq);

        if !self.buckets[self.directory[h]].insert(record.clone()) {
            self.split(h, record);
        }

        true
    }

    fn split(&mut self, dir_index: usize, record: Record) {
        let bucket_index = self.directory[dir_index];

        let bkp = self.buckets[bucket_index].clone();

        // Dobra diretorio se ld = gd
        if bkp.local_depth == self.global_depth {
            self.double_directory();
        }

        // Retira dado do balde e incrementa ld
        self.buckets[bucket_index].data = Vec::new();
        self.buckets[bucket_index].local_depth += 1;

        // Cria balde novo
        self.buckets.push(BucketAlt1::new(
            next_string(&self.buckets.last().unwrap().name.clone()),
            self.buckets[bucket_index].local_depth,
            self.buckets[bucket_index].size,
        ));

        // Mudar ponteiro para balde novo
        for i in (dir_index + 1)..self.directory.len() {
            if self.directory[i] == self.directory[dir_index] {
                self.directory[i] = self.buckets.len() - 1;
                break;
            }
        }

        // Reorganizar entradas
        for i in bkp.data {
            self.insert(i);
        }
        self.insert(record);
    }

    fn double_directory(&mut self) {
        let n = self.directory.len();

        for i in 0..n {
            self.directory.push(self.directory[i])
        }

        self.global_depth += 1;
    }

    pub fn remove(&mut self, key: i32) -> Option<Record> {
        let h: usize = self.hash_fun(key);

        return self.buckets[self.directory[h]].remove(key);
    }

    pub fn search(&self, key: i32) -> Option<(usize, usize)> {
        let h = self.hash_fun(key);

        if let Some(slotid) = self.buckets[self.directory[h]].search(key) {
            return Some((self.directory[h], slotid));
        }

        return None;
    }

    pub fn serialize(&self) -> Vec<u8> {
        // | 1B gd |  2B m | 405B b1 |405B b2 |...|405B bm | 2B n |2B d1 |2B d2 |...|2B dn |

        let mut encoded: Vec<u8> = Vec::new();

        // Global Depth
        encoded.push(self.global_depth.to_be_bytes()[0]);

        // Buckets
        encoded.extend_from_slice(&(self.buckets.len() as u16).to_be_bytes());

        for b in &self.buckets {
            encoded.append(&mut b.serialize());
        }

        // Directory size
        encoded.extend_from_slice(&(self.directory.len() as u16).to_be_bytes());

        for d in &self.directory {
            encoded.extend_from_slice(&(*d as u16).to_be_bytes())
        }

        encoded
    }

    pub fn deserialize(mut f: &mut File) -> Self {
        // Global depth (1B)
        let mut buffer = [0; 1];

        f.read(&mut buffer).unwrap();

        let global_depth = buffer[0];

        // #baldes (2B)
        let mut buffer = [0; 2];

        f.read(&mut buffer).unwrap();

        let m = u16::from_be_bytes(buffer) as usize;

        // Baldes (#baldes * (5 + size * 100))
        let mut buckets: Vec<BucketAlt1> = Vec::new();

        for _ in 0..m {
            buckets.push(BucketAlt1::deserialize(&mut f));
        }

        // #direc
        let mut buffer = [0; 2];

        f.read(&mut buffer).unwrap();

        let n = u16::from_be_bytes(buffer);

        // Diretorios
        let mut directory: Vec<usize> = Vec::new();

        let mut buffer = [0; 2];

        for _ in 0..n {
            f.read(&mut buffer).unwrap();

            directory.push(u16::from_be_bytes(buffer) as usize);
        }

        HashAlt1 {
            global_depth,
            directory,
            buckets,
        }
    }
}

fn next_string(input: &String) -> String {
    let mut chars = input.chars();

    match (chars.next(), chars.next()) {
        (Some('Z'), None) => "AA".to_string(),
        (Some(l), None) => next_letter(l).to_string(),
        (Some('Z'), Some('Z')) => 'A'.to_string(),
        (Some(l), Some('Z')) => {
            let mut s = String::new();

            s.push(next_letter(l));
            s.push('A');

            s
        }
        (Some(l1), Some(l2)) => {
            let mut s = String::new();

            s.push(l1);
            s.push(next_letter(l2));

            s
        }
        (None, _) => 'A'.to_string(),
    }
}

fn next_letter(input: char) -> char {
    ((input as u8) + 1) as char
}

impl fmt::Display for HashAlt1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hash_string = String::new();
        let left_padding = (self.global_depth + 2) as usize;
        let pad = ' '.to_string().repeat(left_padding);
        let small_square_sep = "+---+";
        let big_square_size = 7;
        let big_square_sep = format!("+{}+", "-".to_string().repeat(big_square_size));
        let table_len = left_padding + big_square_size + 4;

        // Print global depth
        hash_string.push_str(format!("{pad}{small_square_sep}\n").as_str());
        hash_string.push_str(format!("{pad}|{: ^3}|\n", self.global_depth).as_str());
        hash_string.push_str(
            format!(
                "{pad}{small_square_sep}{}\n",
                big_square_sep.get(5..).unwrap()
            )
            .as_str(),
        );

        // Print directories
        for i in 0..self.directory.len() {
            hash_string.push_str(
                format!(
                    " {num:0length$b} |",
                    num = i,
                    length = self.global_depth as usize
                )
                .as_str(),
            );
            hash_string.push_str(
                format!(
                    "{: ^big_square_size$}|\n{pad}{big_square_sep}\n",
                    self.buckets[self.directory[i]].name
                )
                .as_str(),
            )
        }

        let mut bucket_string = String::new();

        for b in &self.buckets {
            bucket_string.push_str(format!("{}\n\n", b).as_str())
        }

        let mut hash_lines: Vec<&str> = hash_string.lines().collect();
        let mut buck_lines: Vec<&str> = bucket_string.lines().collect();

        let right_pad = " ".to_string().repeat(table_len);

        if hash_lines.len() < buck_lines.len() {
            for _ in hash_lines.len()..buck_lines.len() {
                hash_lines.push(right_pad.as_str())
            }
        } else if buck_lines.len() < hash_lines.len() {
            for _ in buck_lines.len()..hash_lines.len() {
                buck_lines.push(" ");
            }
        }

        for (h, b) in hash_lines.iter().zip(buck_lines) {
            let right_pad = table_len - h.len();
            writeln!(f, "{h}{}{b}", " ".to_string().repeat(right_pad)).unwrap();
        }

        writeln!(f)
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn test_hash_display_global_depth_2() {
        let h = HashAlt1::new(2, 4);

        println!("{h}");

        println!("{}", u32::MAX);

        assert_eq!(format!("{h}"), "    +---+      +---+\n    | 2 |      | 2 |\n    +---+---+  +---+---+---+---+\n 00 |   A   |  |   |   |   |   | A\n    +-------+  +---+---+---+---+\n 01 |   B   |  \n    +-------+  +---+\n 10 |   C   |  | 2 |\n    +-------+  +---+---+---+---+\n 11 |   D   |  |   |   |   |   | B\n    +-------+  +---+---+---+---+\n               \n               +---+\n               | 2 |\n               +---+---+---+---+\n               |   |   |   |   | C\n               +---+---+---+---+\n               \n               +---+\n               | 2 |\n               +---+---+---+---+\n               |   |   |   |   | D\n               +---+---+---+---+\n               \n\n");
    }

    #[test]
    fn test_hash_display_global_depth_3() {
        let h = HashAlt1::new(3, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "     +---+      +---+\n     | 3 |      | 3 |\n     +---+---+  +---+---+---+---+\n 000 |   A   |  |   |   |   |   | A\n     +-------+  +---+---+---+---+\n 001 |   B   |  \n     +-------+  +---+\n 010 |   C   |  | 3 |\n     +-------+  +---+---+---+---+\n 011 |   D   |  |   |   |   |   | B\n     +-------+  +---+---+---+---+\n 100 |   E   |  \n     +-------+  +---+\n 101 |   F   |  | 3 |\n     +-------+  +---+---+---+---+\n 110 |   G   |  |   |   |   |   | C\n     +-------+  +---+---+---+---+\n 111 |   H   |  \n     +-------+  +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | D\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | E\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | F\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | G\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | H\n                +---+---+---+---+\n                \n\n");
    }

    #[test]
    fn test_hash_display_global_depth_4() {
        let h = HashAlt1::new(4, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "      +---+      +---+\n      | 4 |      | 4 |\n      +---+---+  +---+---+---+---+\n 0000 |   A   |  |   |   |   |   | A\n      +-------+  +---+---+---+---+\n 0001 |   B   |  \n      +-------+  +---+\n 0010 |   C   |  | 4 |\n      +-------+  +---+---+---+---+\n 0011 |   D   |  |   |   |   |   | B\n      +-------+  +---+---+---+---+\n 0100 |   E   |  \n      +-------+  +---+\n 0101 |   F   |  | 4 |\n      +-------+  +---+---+---+---+\n 0110 |   G   |  |   |   |   |   | C\n      +-------+  +---+---+---+---+\n 0111 |   H   |  \n      +-------+  +---+\n 1000 |   I   |  | 4 |\n      +-------+  +---+---+---+---+\n 1001 |   J   |  |   |   |   |   | D\n      +-------+  +---+---+---+---+\n 1010 |   K   |  \n      +-------+  +---+\n 1011 |   L   |  | 4 |\n      +-------+  +---+---+---+---+\n 1100 |   M   |  |   |   |   |   | E\n      +-------+  +---+---+---+---+\n 1101 |   N   |  \n      +-------+  +---+\n 1110 |   O   |  | 4 |\n      +-------+  +---+---+---+---+\n 1111 |   P   |  |   |   |   |   | F\n      +-------+  +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | G\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | H\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | I\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | J\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | K\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | L\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | M\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | N\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | O\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | P\n                 +---+---+---+---+\n                 \n\n");
    }

    #[test]
    fn test_hash_fun_2() {
        let h = HashAlt1::new(2, 4);

        assert_eq!(h.hash_fun(343), 3)
    }

    #[test]
    fn test_hash_fun_3() {
        let h = HashAlt1::new(3, 4);

        assert_eq!(h.hash_fun(343), 7)
    }

    #[test]
    fn test_insert_global_depth_2() {
        let mut h = HashAlt1::new(2, 4);

        println!("{h}");

        h.insert(Record{nseq: 2, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 10, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 102, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 98, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 118, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});

        println!("{h}");

        h.insert(Record{nseq: 0, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 4, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 12, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 20, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});
        h.insert(Record{nseq: 24, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()});

        println!("{h}");

        let s1 = h.search(0).unwrap();
        let s2 = h.search(14);
        assert_eq!(h.buckets[s1.0].data[s1.1], Record{nseq: 0_i32, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.\0".to_string()});
        assert_eq!(h.search(14), None)
    }

    #[test]
    fn test_hash_double_directory() {
        let mut h = HashAlt1::new(2, 4);

        println!("{h}");

        h.double_directory();

        println!("{h}");

        assert_eq!(1, 1);
    }

    #[test]
    fn test_insert_split_double() {
        let mut h = HashAlt1::new(2, 4);

        // 0, 8, 24, 56, 120

        h.insert(Record{nseq: 0, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 8, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 24, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 56, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});

        println!("{h}");

        h.insert(Record{nseq: 120, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});

        println!("{h}");
    }

    #[test]
    fn test_serialize_hash() {
        let mut h = HashAlt1::new(2, 4);

        h.insert(Record{nseq: 0, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 1, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 2, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});
        h.insert(Record{nseq: 3, text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque imperdiet lacinia orci aliquam.".to_string()});

        let encoded = h.serialize();

        let mut file = File::create("index.bin").unwrap();
        file.write_all(&encoded).unwrap();

        assert_eq!("_", "_")
    }

    #[test]
    fn test_deserialize() {
        let mut f = File::open("index.bin").unwrap();

        let h = HashAlt1::deserialize(&mut f);

        println!("{h}");

        assert_eq!("", "")
    }
}
