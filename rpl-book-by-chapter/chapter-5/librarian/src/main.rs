struct Book {
    title: String,
    isbn: u64,
    year_published: i32,
}

impl Book {
    fn new() -> Book {
        Book {
            title: "Untitled Book".to_string(),
            isbn: 1234567890123,
            year_published: 2023,
        }
    }

    fn get_book_info(book: &Book) {
        println!("{}t.. wait, {}? Oh, I've heard of that one!", &book.title[0..2], book.title);
        println!("{} was made in {}. I think this one has an ISBN of {}...", book.title, book.year_published, book.isbn);
    }
}

fn main() {
    let charlottes_web = Book {
        title: "Charlotte's Web".to_string(),
        isbn: 9780064410939,
        year_published: 1952,
    };

    println!("> Excuse me! Do you happen to know about this book?");

    Book::get_book_info(&charlottes_web);

    println!("\n> How about this one? It looks weird...");
    Book::get_book_info(&Book::new());

    println!("But I don't think you're supposed to see that one..!");
}
