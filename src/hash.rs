use crate::bucket::*;
use core::fmt;

struct Hash {
    global_depth: usize,
    buckets: Vec<Bucket>,
    directories: Vec<Vec<usize>>,
}

impl Hash {
    pub fn new(global_depth: usize, bucket_size: usize) -> Self {
        let size = 2_u32.pow(global_depth as u32) as usize;

        let mut directories = Vec::with_capacity(size);

        let mut buckets = Vec::with_capacity(size);

        for i in 0..size {
            directories.push(vec![i]);
            buckets.push(Bucket::new(
                ((b'A' + i as u8) as char).to_string(),
                global_depth,
                bucket_size,
            ));
        }

        Hash {
            global_depth,
            buckets,
            directories,
        }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hash_string = String::new();
        let left_padding = self.global_depth + 2;
        let pad = ' '.to_string().repeat(left_padding);
        let small_square_sep = "+---+";
        let big_square_size = self.buckets.len() * 2 + 2;
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
        for i in 0..self.directories.len() {
            hash_string.push_str(
                format!(" {num:0length$b} |", num = i, length = self.global_depth).as_str(),
            );
            let mut tmp = String::new();
            tmp.push('(');
            for j in &self.directories[i] {
                tmp.push_str(format!("{},", self.buckets[*j].name).as_str());
            }
            tmp.push(')');
            hash_string
                .push_str(format!("{tmp: <big_square_size$}|\n{pad}{big_square_sep}\n").as_str())
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
    use super::*;

    #[test]
    fn test_hash_display_global_depth_2() {
        let h = Hash::new(2, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "\t+---+\n\t| 2 |\n");
    }

    #[test]
    fn test_hash_display_global_depth_3() {
        let h = Hash::new(3, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "_");
    }

    #[test]
    fn test_hash_display_global_depth_4() {
        let h = Hash::new(4, 4);

        println!("{h}");

        assert_eq!(format!("{h}"), "_");
    }
}
