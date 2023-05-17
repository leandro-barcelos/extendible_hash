use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Bucket {
    pub name: String,
    pub local_depth: usize,
    pub data: Vec<(i32, String)>,
    pub size: usize,
}

impl Bucket {
    pub fn new(name: String, local_depth: usize, size: usize) -> Self {
        Bucket {
            name,
            local_depth,
            data: Vec::with_capacity(size),
            size,
        }
    }

    pub fn insert(&mut self, record: (i32, String)) -> bool {
        if self.data.len() == self.size {
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
}

impl Display for Bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        // Local depth
        let depth_square = format!("+---+\n|{: ^3}|\n", self.local_depth);
        s.push_str(format!("{}", depth_square).as_str());

        // Bucket
        let sep = format!("+{}", "---+".to_string().repeat(self.size));
        s.push_str(format!("{sep}\n|").as_str());

        for i in 0..self.size {
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
}
