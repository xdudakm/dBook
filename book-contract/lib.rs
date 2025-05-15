#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod voting {
    use ink::codegen::Env;
    use ink::storage::Mapping;
    use ink::prelude::{
        vec::Vec,
        string::String,
    };

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Book {
        id: u32,
        owner: AccountId,
        title: String,
        author: String,
        content: String,
        price: u32,
        for_sale: bool,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct BookInit {
        title: String,
        author: String,
        content: String,
        price: u32,
        for_sale: bool,
    }

    #[ink(storage)]
    pub struct BookStore {
        books: Mapping<u32, Book>,
        num_books: u32,
    }

    impl BookStore {
        #[ink(constructor)]
        pub fn new(names: Vec<BookInit>) -> Self {
            let creator = Self::env().caller();
            let mut books = Mapping::default();
            for (i, book) in names.iter().enumerate() {
                let i: u32 = i.try_into().expect("value doesn't fit into u32");
                books.insert(i, &Book {
                    id: i,
                    owner: creator,
                    title: book.title.clone(),
                    author: book.author.clone(),
                    content: book.content.clone(),
                    price: book.price,
                    for_sale: false,
                });
            }
            let num_books = names.len().try_into().expect("value doesn't fit into u32");
            Self {
                books,
                num_books,
            }
        }

        #[ink(message)]
        pub fn add_book(&mut self, title: String, author: String, content: String, price: u32) {
            let seller = self.env().caller();
            let book = Book {
                id: self.num_books,
                owner: seller,
                title,
                author,
                content,
                price,
                for_sale: false,
            };

            self.books.insert(self.num_books, &book);
            self.num_books = self.num_books.saturating_add(1);
        }

        #[ink(message)]
        pub fn buy(&mut self, book_id: u32) {
            let caller = self.env().caller();
            let mut book = self.books.get(book_id).expect("Book not found");
            assert!(book.for_sale, "The book is not for sale");
            assert_ne!(book.owner, caller, "The book is already owned");
            book.owner = caller;
            book.for_sale = false;
            self.books.insert(book_id, &book);
        }

        #[ink(message)]
        pub fn sell(&mut self, book_id: u32) {
            let caller = self.env().caller();
            let mut book = self.books.get(book_id).expect("Book not found");
            assert_eq!(book.owner, caller, "You don't own the book");
            book.for_sale = true;
            self.books.insert(book_id, &book);
        }

        #[ink(message)]
        pub fn get_content(&mut self, book_id: u32) -> String {
            let caller = self.env().caller();
            let book = self.books.get(book_id).expect("Book not found");
            assert_eq!(book.owner, caller, "You don't own the book");
            book.content
        }

        #[ink(message)]
        pub fn owned_books(&self) -> Vec<Book> {
            let caller = self.env().caller();
            let mut list = Vec::new();
            for i in 0..self.num_books {
                if let Some(book) = self.books.get(i) {
                    if book.owner == caller {
                        list.push(book);
                    }
                }
            }
            list
        }

        #[ink(message)]
        pub fn books_for_sale(&self) -> Vec<Book> {
            let caller = self.env().caller();
            let mut list = Vec::new();
            for i in 0..self.num_books {
                if let Some(book) = self.books.get(i) {
                    if book.for_sale && book.owner != caller {
                        list.push(book);
                    }
                }
            }
            list
        }
    }
}
