// Struct Challenge
/*
    Objectives
    You are required to implement a few functionalities in the Book struct:

    - Add an associated function called new() that creates a new book instance with a default value of available set to true.
    - Add a borrow() method to set the available field to false.
    - Add a return_book() method to set the available field to true.
    - Add a method is_available() that returns a boolean indicating if the book is available or not.
    - Create a few book instances and demonstrate borrow and returning books.
*/

fn main() {
    // test instantiation
    let mut book = Book::new(
        "There and Back Again: A Hobbit's Tale",
        "Bilbo Baggins");

    println!("{:?}", book);

    // test borrow
    book.borrow();
    println!(
        "{} by {} is available? {}",
        book.title,
        book.author,
        book.available);

    // test return
    book.return_book();
    println!(
        "{} by {} is available? {}",
        book.title,
        book.author,
        book.available);

    // test availability check
    let mut book_2 = Book::new(
        "The Lord of The Rings",
        "J.R.R. Tolkien"
    );
    println!("{} is available?: {}", book_2.title, book_2.is_available());

    book_2.borrow();
    println!("{} is available?: {}", book_2.title, book_2.is_available());

    book_2.return_book();
    println!("{} is available?: {}", book_2.title, book_2.is_available());

}

#[derive(Debug)] // use macro on struct so that we can log out struct instances to the console
struct Book {
    title: String,
    author: String,
    available: bool,
}

// implement methods & associated functions
impl Book {
    // associated function to create new book instances
    fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            available: true,
        }
    }

    // borrow method to set available to false
    fn borrow(&mut self) {
        self.available = false;

        println!("{} by {} has been checked out", self.title, self.author);
    }

    // return_book method to set available to true
    fn return_book(&mut self) {
        self.available = true;

        println!("{} by {} has been returned", self.title, self.author);
    }

    // is_available method to check availability
    fn is_available(&self) -> bool {
        self.available // return availability
    }
}