mod bootes;

pub use bootes::def;

#[cfg(test)]
mod tests {
    #[test]
    fn test_user() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let user1 = super::def::User::register("aaa", "aaa@ll.com");
        assert_eq!(user1.valid(), true);
        let mut user2 = super::def::User::register("", "a@l.com");
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
}
