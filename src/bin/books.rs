use idioms::structures::{make_sample_books, BookBuilder};

fn main() {
    println!("Stuff with books");

    let books_data = make_sample_books(5, 10);
    let books_data_len = books_data.len();

    let builder = BookBuilder::new(books_data);

    let mut books = builder.build();

    println!(
        "Have created {} books from {} lines of book data",
        books.len(),
        books_data_len
    );

    books.sort_unstable_by(|b1, b2| b1.title.cmp(&b2.title));

    println!("Book titles in order are:\n");
    for book in &books {
        println!("'{}', {} pages", book.title, book.num_pages())
    }
}
