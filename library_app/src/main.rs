use library_inventory::{Book, Genre, Inventory};
use library_users::{User, UserManager};
use library_borrowing::BorrowingService;
use std::io::{self, Write};

 

fn main() {
   let mut inventory = Inventory::new();

   let mut user_manager = UserManager::new();

   let mut borrow_service = BorrowingService::new();

   loop {
    println!("\n Library management System");
    println!("1. Add a Book");
    println!("2. Register User");
    println!("3. Borrow Book");
    println!("4. Return Book");
    println!("5. List Books");
    println!("6. Exist");

    println!("Enter Your Choice");

    io::stdout().flush().unwrap();
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let mut choice: u32 = choice.trim().parse::<u32>().unwrap_or(0);

    match choice {
        1 => {
            println!("Enter Book Details");
            println!("Id: ");
            let mut id: String = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let mut id: u32 = id.trim().parse::<u32>().unwrap_or(0);

            println!("Title: ");
            io::stdout().flush().unwrap();
            let mut title: String = String::new();
            io::stdin().read_line(&mut title).unwrap();

             println!("Author: ");
            io::stdout().flush().unwrap();
            let mut author: String = String::new();
            io::stdin().read_line(&mut author).unwrap();
            
            let book: Book = Book {
                id: id,
                title: title,
                author: author,
                genre: Genre::Other(String::from("Other")),
                is_available: true,
            };

            inventory.add_book(book);

            println!("Book Added");

        },
        2 => {
              println!("Enter User Details");
            println!("Id: ");
            io::stdout().flush().unwrap();
            let mut id: String = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let mut id: u32 = id.trim().parse::<u32>().unwrap_or(0);

             println!("Name: ");
            io::stdout().flush().unwrap();
            let mut name: String = String::new();
            io::stdin().read_line(&mut name).unwrap();

            let user: User = User {
                id: id,
                name: name,
                borrowed_books: Vec::new()
            };
            user_manager.register_user(user);
            println!("New User Added")
        },
        3 => {
            println!("Borrow a Book");
            println!("Enter Book Id: ");
            io::stdout().flush().unwrap();
            let mut book_id: String = String::new();
            io::stdin().read_line(&mut book_id).unwrap();
            let mut book_id: u32 = book_id.trim().parse::<u32>().unwrap_or(0);

             println!("Enter User Id: ");
             io::stdout().flush().unwrap();
            let mut user_id: String = String::new();
            io::stdin().read_line(&mut user_id).unwrap();
            let mut user_id: u32 = user_id.trim().parse::<u32>().unwrap_or(0);

           match borrow_service.borrow_book(&mut  inventory, &mut user_manager, user_id, book_id) {
            Ok(_) => println!("Book Borrowed Succcessfully"),
            Err(err) => println!("Error: {}", err),
           }



        },
        4 => {
            println!("Returned Book");
             println!("Enter Book Id: ");
             io::stdout().flush().unwrap();
            let mut book_id: String = String::new();
            io::stdin().read_line(&mut book_id).unwrap();
            let mut book_id: u32 = book_id.trim().parse::<u32>().unwrap_or(0);

             println!("Enter User Id: ");
             io::stdout().flush().unwrap();
            let mut user_id: String = String::new();
            io::stdin().read_line(&mut user_id).unwrap();
            let mut user_id: u32 = user_id.trim().parse::<u32>().unwrap_or(0);

            match borrow_service.return_book(&mut inventory,  &mut user_manager, user_id, book_id) {
                Ok(_) => println!("Book Returned Succcessfully"),
                Err(err) => println!("Error: {}", err),
            }

            println!("Book Returned");  
        },
        5 => {
            println!("List of Books");
           let books =  inventory.list_books();
            for book in books {
                println!("ID: {}, Name: {}, Author: {}, Avalailable: {}, ", book.id, book.title, book.author, book.is_available,)
            }
           //  println!("List of Books: {}", books); 
        },
        6 => {
            println!("Existing.....");
            break;
        },
        _ => {
            println!("Invalid choice, Please  Try Again")
        }

    }
   }
}
