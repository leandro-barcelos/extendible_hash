use core::fmt;

struct Hash {
    global_depth: usize,
    directories: Vec<Box<Bucket>>,
}

struct Bucket {
    local_depth: usize,
    data: Vec<Key>,
}

enum Key {
    Primary(i32),
    Secondary(String, i32),
}

impl Hash {
    fn new(global_depth: usize, bucket_size: usize) -> Self {
        let mut directories = Vec::new();

        for _ in 0..2_u32.pow(global_depth as u32) as usize {
            directories.push(Box::new(Bucket::new(global_depth, bucket_size)))
        }

        Hash {
            global_depth,
            directories,
        }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l
    }
}

impl Bucket {
    fn new(local_depth: usize, size: usize) -> Self {
        Bucket {
            local_depth,
            data: Vec::with_capacity(size),
        }
    }
}
