fn main() {
    let book = Book {
        pages: 105,
        rating: 7,
    };
    display_book_rating(&book);
    display_page_count(&book); // Ownership went from main -> display_book_rating -> dropped. Hence, can not use book again for display_page_count
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("Number of Pages: {:?}", book.pages);
}

fn display_book_rating(book: &Book) {
    println!("Book Rating: {:?}", book.rating);
}