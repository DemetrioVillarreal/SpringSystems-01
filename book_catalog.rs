use std::fs::File;

use std::io::{Write, BufReader, BufRead};

struct Book {

    title: String,

    author: String,

    year: u16,

    
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Create (or overwrite) the file
    let mut file = File::create(filename).unwrap();

    // Loop through each book
    for book in books {
        // Write: title,author,year
        writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new();

    // Read file line by line
    for line in reader.lines() {
        let line = line.unwrap();

        // Split by comma
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2].parse().unwrap();

            let book = Book {
                title,
                author,
                year,
            };

            books.push(book);
        }
    }

    books
}

fn main() {
    let books = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");

    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}