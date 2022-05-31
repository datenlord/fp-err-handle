pub trait Monoid {
    fn new() -> Self;
    fn append(&mut self, other: Self) -> Self;
}

impl Monoid for String {
    fn new() -> Self {
        String::new()
    }
    
    fn append(&mut self, other: Self) -> Self {
        self.push_str(&other);
        self.to_string()
    }
}