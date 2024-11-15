
#[derive(Debug, Clone)]
pub enum Genre {
Fiction,
NonFiction,
Science,
History,
Fantasy,
Other(String)
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: Genre,
    pub is_available: bool
}


pub struct Inventory {
    books: Vec<Book>,
}

impl Inventory {
    pub fn new () -> Self {
        Self {
            books: Vec::new()
        }
    }

    pub fn add_book (&mut self, book: Book) {
        self.books.push(book)
    }

    pub fn remove_book (&mut self, book_id: u32) -> Option<Book> {
         if let Some(pos) = self.books.iter().position(|b| b.id == book_id){
           return Some(self.books.remove(pos));
         } else {
          return None;
         }
    }

    pub fn update_book_availability(&mut self, book_id: u32, is_available: bool) -> Result<(), String> {
        if let Some(book ) = self.books.iter_mut().find(|b| b.id == book_id) {
            book.is_available = is_available;
            Ok(())
        } else {
            Err(String::from("Book Not Found"))
        }

    }

    pub fn get_book (&self, book_id: u32) ->  Option<&Book> {
        self.books.iter().find(|&b| b.id == book_id)
    }

    pub fn list_books (&self) -> &Vec<Book> {
        &self.books
    }
}


        //  data moved here
        // move occurs because `b` has type `&mut Book`, which does not implement the `Copy` trait




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_book() {
        let mut inventory: Inventory = Inventory::new();

        let book: Book = Book {
            id: 1,
            title: "Smart move".to_string(),
            author: String::from("Grindle"),
            is_available: true,
            genre: Genre::Science
        };

        inventory.add_book(book);

        assert_eq!(inventory.get_book(1).unwrap().title, "Smart move");

        let remove_book = inventory.remove_book(1);
        assert!(remove_book.is_some());
        assert!(inventory.get_book(1).is_none());
    }
}
