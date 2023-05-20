pub struct User {
    pub name: String,
    pub age: i32,
    pub height: f32,
    pub gads: Gadgets
}

impl User {
    pub fn is_tall(&self) -> bool {
        self.height >= 180.0
    }

    pub fn compare_height(&self, other: &User) -> bool {
        self.height > other.height
    }
}

pub struct Gadgets {
    pub tv: i32,
    pub phone: i32
}