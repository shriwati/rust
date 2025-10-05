#[allow(unused)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


impl User {
    fn change_username(&mut self, username: &str) {
       self.username = String::from(username); 
    }
}



#[warn(unused_imports)]
#[cfg(test)]
mod tests {
    use crate::m2_struct::User;

    #[test]
    fn test_struct() {
        let user1 = User {
            username:"shri".to_string(),
            email:"shrisakrikar@hotmail.com".to_string(),
            sign_in_count:0,
            active:true,
        };
        dbg!(user1);
    }
    #[test]
    fn test_change_username() {
        let mut user:User = User{
            username:"shri".to_string(),
            email:"shrisakrikar@hotmail.com".to_string(),
            sign_in_count:0,
            active:true,
        };
         user.change_username("shri");
         assert_eq!(user.username, String::from("shri"));
         dbg!(user);
    }
    
}