

use library_inventory::{Book, Inventory};
use library_users::{ User, UserManager};


// pub struct BorrowingService<'a> {
//     inventory: &'a mut Inventory,
//     user_manager: &'a mut UserManager,
// }

pub struct BorrowingService {

}

impl BorrowingService{

    pub fn new() -> Self {
        Self {}
    }

    pub fn borrow_book(&mut self, inventory: &mut Inventory, user_manager: &mut UserManager, user_id: u32, book_id: u32) -> Result<(), String> {
        // check if the book exist
        let book  = inventory.get_book(book_id).ok_or_else(|| String::from("Book Not Found"))?;

        // check if the book is available
        if !book.is_available {
           return  Err(String::from("Book is not available"));
        }
        // 
        let user = user_manager.get_user(user_id).ok_or_else(|| String::from("User Not Found"))?;
        
        // user cannot borrow more than 3 books
        if user.borrowed_books.len() >= 3 {
             return Err(String::from("Cannot Borrow More than three books"));
        }

        // make book not available
       let _ = inventory.update_book_availability(book_id, false);
       let _ = user_manager.borrow_book(book_id, user_id);

        Ok(())
    }

    pub fn return_book(&mut self,  inventory: &mut Inventory, user_manager: &mut UserManager,  user_id: u32, book_id: u32) -> Result<(), String> { 
        user_manager.return_book(book_id, user_id);
       let _ = inventory.update_book_availability(book_id, true);
        Ok(())
    }
   
}



#[cfg(test)]
mod tests {
    use super::*;

use library_inventory::{Book, Inventory, Genre};
use library_users::{ User, UserManager};

    fn setup() -> (UserManager, Inventory, BorrowingService) {
        let mut inventory = Inventory::new();
        let mut user_manager = UserManager::new();
        let mut borrow_service = BorrowingService::new();

        let book: Book = Book {
            id: 1,
            title: String::from("Rust Book"),
            author: String::from("Steve"),
            genre: Genre::Science,
            is_available: true
         };
       // add book
       inventory.add_book(book);
       
       let user : User = User {
        id: 2,
        name: String::from("Mubarak"),
        borrowed_books: Vec::new()
       };

       // add user 
       user_manager.register_user(user);
       
       (user_manager, inventory, borrow_service)

    }

    #[test]
    fn test_borrow_and_return_book() {
        let (mut user_manager, mut inventory, mut borrow_service) = setup();

        assert!(borrow_service.borrow_book(&mut inventory, &mut user_manager, 2, 1).is_ok());

        assert!(!inventory.get_book(1).unwrap().is_available);

        assert!(user_manager.get_user(2).unwrap().borrowed_books.contains(& 1));

        // check for return book
        assert!(borrow_service.return_book(&mut inventory, &mut user_manager, 2, 1).is_ok());

        // asset the book is available after it was return
        assert!(inventory.get_book(1).unwrap().is_available);
    }
}
