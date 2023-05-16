use std::fmt::{write, Display};

pub struct Bucket {
    pub name: String,
    local_depth: usize,
    data: Vec<usize>,
    size: usize,
}

// enum Key {
//     Primary(i32),
//     Secondary(String, i32),
// }

impl Bucket {
    pub fn new(name: String, local_depth: usize, size: usize) -> Self {
        Bucket {
            name,
            local_depth,
            data: Vec::with_capacity(size),
            size,
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
        let sep = format!("+{}", "---+".to_string().repeat(self.size));
        s.push_str(format!("{sep}\n|").as_str());

        for i in 0..self.size {
            if let Some(d) = self.data.get(i) {
                s.push_str(format!("{: ^3}|", d).as_str())
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
