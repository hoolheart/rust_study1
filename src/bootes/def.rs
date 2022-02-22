
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
}

impl ToString for IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(x0, x1, x2, x3) => format!("{}.{}.{}.{}", x0, x1, x2, x3),
            IpAddr::V6(x0, x1, x2, x3, x4, x5, x6, x7) => format!("{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}", x0, x1, x2, x3, x4, x5, x6, x7),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width) && (self.height >= other.height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let user1 = super::User::register("aaa", "aaa@ll.com");
        assert_eq!(user1.valid(), true);
        let mut user2 = super::User::register("", "a@l.com");
        assert_eq!(user2.valid(), false);
        user2.name = String::from("oh");
        assert_eq!(user2.valid(), true);
        for _ in 0..10 {
            user2.increase();
        }
        assert_eq!(user2.count_sign_in(), 11);
        user2.withdraw();
        assert_eq!(user2.valid(), false);
    }

    #[test]
    fn test_rectangle() {
        let rect0 = super::Rectangle{
            width: 100,
            height: 70,
        };
        let rect1 = super::Rectangle{
            width: 60,
            height: 70,
        };
        assert!(rect0.can_hold(&rect1));
    }
}
