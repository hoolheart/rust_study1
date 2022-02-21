
#[derive(Debug)]
pub struct User {
    pub active : bool,
    pub email: String,
    pub name: String,
    sign_in_count : u64,
}

impl User {
    pub fn register(name: &str, email: &str) -> User {
        User {
            active: true,
            email: String::from(email),
            name: String::from(name),
            sign_in_count: 1,
        }
    }
    pub fn valid(&self) -> bool {
        self.active && (self.email.len() > 0) && (self.name.len() > 0)
    }
    pub fn increase(&mut self) -> u64 {
        self.sign_in_count += 1;
        self.sign_in_count
    }
    pub fn count_sign_in(&self) -> u64 {
        self.sign_in_count
    }
    pub fn withdraw(&mut self) {
        self.active = false
    }
}

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}

impl IpAddr {
    pub fn version(&self) -> i32 {
        match self {
            IpAddr::V4(..) => 4,
            IpAddr::V6(..) => 6,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            IpAddr::V4(x0, x1, x2, x3) => format!("{}.{}.{}.{}", x0, x1, x2, x3),
            IpAddr::V6(x0, x1, x2, x3, x4, x5, x6, x7) => format!("{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}", x0, x1, x2, x3, x4, x5, x6, x7),
        }
    }
}
