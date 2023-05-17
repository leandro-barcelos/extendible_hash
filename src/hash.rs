// use bincode::serialize;
// use serde::{Deserialize, Serialize};

use crate::bucket::*;
use core::fmt;
use std::{cell::RefCell, rc::Rc};

// #[derive(Serialize, Deserialize)]
pub struct Hash {
    global_depth: usize,
    directory: Vec<Rc<RefCell<Bucket>>>,
    buckets: Vec<Rc<RefCell<Bucket>>>,
}

impl Hash {
    pub fn new(global_depth: usize, bucket_size: usize) -> Self {
        let size = 2_u32.pow(global_depth as u32) as usize;

        let mut directory = Vec::with_capacity(size);
        let mut buckets = Vec::with_capacity(size);

        for i in 0..size {
            directory.push(Rc::new(RefCell::new(Bucket::new(
                ((b'A' + i as u8) as char).to_string(),
                global_depth,
                bucket_size,
            ))));
            buckets.push(directory[i].clone());
        }

        Hash {
            global_depth,
            directory,
            buckets,
        }
    }

    pub fn hash_fun(&self, num: i32) -> usize {
        let mask = (1 << self.global_depth) - 1;
        (num & mask) as usize
    }

    pub fn insert(&mut self, mut record: (i32, String)) {
        if record.1.len() < 96 {
            record.1 = format!(
                "{}{}",
                record.1,
                "\0".to_string().repeat(96 - record.1.len())
            )
        }

        let h = self.hash_fun(record.0);

        if !self.directory[h].borrow_mut().insert(record.clone()) {
            self.split(h);
            self.insert(record)
        }
    }

    fn split(&mut self, bucket_index: usize) {
        let bucket = &self.directory[bucket_index];
        let bkp = bucket.borrow().clone();
        bucket.replace(Bucket::new(bkp.name.clone(), bkp.local_depth + 1, bkp.size));

        if bkp.local_depth == self.global_depth {
            self.double_directory();
        }

        let new_bucket = Bucket::new(
            bkp.name + bkp.local_depth.to_string().as_str(),
            bkp.local_depth + 1,
            bkp.size,
        );

        self.buckets.push(Rc::new(RefCell::new(new_bucket)));

        self.directory[bucket_index + 2_u32.pow(bkp.local_depth as u32) as usize] =
            self.buckets.last().unwrap().clone();

        for i in bkp.data {
            self.insert(i);
        }
    }

    fn double_directory(&mut self) {
        let n = self.directory.len();

        for i in n..2 * n {
            let j = self.hash_fun(i as i32);

            self.directory.push(self.directory[j].clone())
        }

        self.global_depth += 1;
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let h: usize = self.hash_fun(key);

        return self.directory[h].borrow_mut().remove(key);
    }

    pub fn search(&self, key: i32) -> Option<(i32, String)> {
        let h = self.hash_fun(key);

        return self.directory[h].borrow().search(key);
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hash_string = String::new();
        let left_padding = self.global_depth + 2;
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
                format!(" {num:0length$b} |", num = i, length = self.global_depth).as_str(),
            );
            hash_string.push_str(
                format!(
                    "{: ^big_square_size$}|\n{pad}{big_square_sep}\n",
                    self.directory[i].borrow().name
                )
                .as_str(),
            )
        }

        let mut bucket_string = String::new();

        for b in &self.buckets {
            bucket_string.push_str(format!("{}\n\n", b.borrow()).as_str())
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
    use std::{fs::File, io::Write, vec};

    use super::*;

    #[test]
    fn test_hash_display_global_depth_2() {
        let h = Hash::new(2, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "    +---+      +---+\n    | 2 |      | 2 |\n    +---+---+  +---+---+---+---+\n 00 |   A   |  |   |   |   |   | A\n    +-------+  +---+---+---+---+\n 01 |   B   |  \n    +-------+  +---+\n 10 |   C   |  | 2 |\n    +-------+  +---+---+---+---+\n 11 |   D   |  |   |   |   |   | B\n    +-------+  +---+---+---+---+\n               \n               +---+\n               | 2 |\n               +---+---+---+---+\n               |   |   |   |   | C\n               +---+---+---+---+\n               \n               +---+\n               | 2 |\n               +---+---+---+---+\n               |   |   |   |   | D\n               +---+---+---+---+\n               \n\n");
    }

    #[test]
    fn test_hash_display_global_depth_3() {
        let h = Hash::new(3, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "     +---+      +---+\n     | 3 |      | 3 |\n     +---+---+  +---+---+---+---+\n 000 |   A   |  |   |   |   |   | A\n     +-------+  +---+---+---+---+\n 001 |   B   |  \n     +-------+  +---+\n 010 |   C   |  | 3 |\n     +-------+  +---+---+---+---+\n 011 |   D   |  |   |   |   |   | B\n     +-------+  +---+---+---+---+\n 100 |   E   |  \n     +-------+  +---+\n 101 |   F   |  | 3 |\n     +-------+  +---+---+---+---+\n 110 |   G   |  |   |   |   |   | C\n     +-------+  +---+---+---+---+\n 111 |   H   |  \n     +-------+  +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | D\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | E\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | F\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | G\n                +---+---+---+---+\n                \n                +---+\n                | 3 |\n                +---+---+---+---+\n                |   |   |   |   | H\n                +---+---+---+---+\n                \n\n");
    }

    #[test]
    fn test_hash_display_global_depth_4() {
        let h = Hash::new(4, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "      +---+      +---+\n      | 4 |      | 4 |\n      +---+---+  +---+---+---+---+\n 0000 |   A   |  |   |   |   |   | A\n      +-------+  +---+---+---+---+\n 0001 |   B   |  \n      +-------+  +---+\n 0010 |   C   |  | 4 |\n      +-------+  +---+---+---+---+\n 0011 |   D   |  |   |   |   |   | B\n      +-------+  +---+---+---+---+\n 0100 |   E   |  \n      +-------+  +---+\n 0101 |   F   |  | 4 |\n      +-------+  +---+---+---+---+\n 0110 |   G   |  |   |   |   |   | C\n      +-------+  +---+---+---+---+\n 0111 |   H   |  \n      +-------+  +---+\n 1000 |   I   |  | 4 |\n      +-------+  +---+---+---+---+\n 1001 |   J   |  |   |   |   |   | D\n      +-------+  +---+---+---+---+\n 1010 |   K   |  \n      +-------+  +---+\n 1011 |   L   |  | 4 |\n      +-------+  +---+---+---+---+\n 1100 |   M   |  |   |   |   |   | E\n      +-------+  +---+---+---+---+\n 1101 |   N   |  \n      +-------+  +---+\n 1110 |   O   |  | 4 |\n      +-------+  +---+---+---+---+\n 1111 |   P   |  |   |   |   |   | F\n      +-------+  +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | G\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | H\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | I\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | J\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | K\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | L\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | M\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | N\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | O\n                 +---+---+---+---+\n                 \n                 +---+\n                 | 4 |\n                 +---+---+---+---+\n                 |   |   |   |   | P\n                 +---+---+---+---+\n                 \n\n");
    }

    #[test]
    fn test_hash_fun_2() {
        let h = Hash::new(2, 4);

        assert_eq!(h.hash_fun(343), 3)
    }

    #[test]
    fn test_hash_fun_3() {
        let h = Hash::new(3, 4);

        assert_eq!(h.hash_fun(343), 7)
    }

    #[test]
    fn test_insert_global_depth_2() {
        let mut h = Hash::new(2, 4);

        println!("{h}");

        h.insert((2, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((10, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((102, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((98, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((118, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));

        println!("{h}");

        h.insert((0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((4, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((12, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((20, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));
        h.insert((24, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.".to_string()));

        println!("{h}");

        assert_eq!(h.search(0), Some((0_i32, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis quis fringilla diam. Duis in est.\0".to_string())));
        assert_eq!(h.search(14), None)
    }

    #[test]
    fn test_hash_double_directory() {
        let mut h = Hash::new(2, 4);

        println!("{h}");

        h.double_directory();

        println!("{h}");

        assert_eq!(1, 1);
    }

    // #[test]
    // fn test_serialize_hash() {
    //     let mut h = Hash::new(2, 4);

    //     h.insert(2);
    //     h.insert(10);
    //     h.insert(102);
    //     h.insert(98);

    //     let encoded = serialize(&h).unwrap();

    //     let mut file = File::create("index.bin").unwrap();
    //     file.write_all(&encoded).unwrap();

    //     assert_eq!("_", "_")
    // }
}
