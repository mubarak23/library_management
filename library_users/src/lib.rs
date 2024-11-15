
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub borrowed_books: Vec<u32>,
}

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Vec::new()
        }
    }

    pub fn register_user(&mut self, user: User) {
        self.users.push(user);
       //  let borrow: Vec<u32> = vec[2,4,6,8];
       
    }

    pub fn get_user(&self, user_id: u32) -> Option<&User> {
        self.users.iter().find(|&u| u.id == user_id)
    }

    pub fn borrow_book(&mut self, book_id: u32, user_id: u32) {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == user_id){
            user.borrowed_books.push(book_id)
        }
    }

    pub fn return_book(&mut self, book_id: u32, user_id: u32) {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == user_id){
             user.borrowed_books.retain(|&id| id != book_id)
        }
    }
}
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_borrow_book(){
        let mut user_manager: UserManager = UserManager::new();

        let user: User = User {
            id: 1,
            name: String::from("Smart Move"),
            borrowed_books: Vec::new()
        };
        user_manager.register_user(user.clone());

        // assertion for register user
        assert_eq!(user_manager.get_user(1).unwrap().name, "Smart Move");

        // assertion for borrowerd book
        user_manager.borrow_book(143, 1);
        assert!(user_manager.get_user(1).unwrap().borrowed_books.contains(&143));
    }
}
