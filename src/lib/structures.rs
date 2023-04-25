#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub number: u64,
    pub words: u32,
    pub text: String,
}

type BookPage = (String, String);
type BookPages = Vec<BookPage>;

impl Book {
    fn new(title: String, pages: Vec<Page>) -> Self {
        Book { title, pages }
    }

    fn num_pages(&self) -> u32 {
        self.pages.len() as u32
    }
}

use itertools::Itertools;
use std::collections::HashMap;

impl From<BookPages> for Book {
    fn from(book_pages: BookPages) -> Book {
        let mut map: HashMap<String, Vec<Page>> = HashMap::new();

        for (book, page) in book_pages {
            if !map.contains_key(&book) {
                map.insert(book.clone(), Vec::new());
            }

            if let Some(book_pages) = map.get_mut(&book) {
                let page_number = (book_pages.len() + 1) as u64;

                book_pages.push(Page::new(page_number, page));
            }
        }

        if map.len() == 1 {
            let book = map.keys().nth(0).unwrap();
            let pages = map.get(book).unwrap().clone();

            Book::new(book.clone(), pages)
        } else {
            Book::new("Empty".to_string(), Vec::<Page>::new())
        }
    }
}

impl Page {
    fn new(number: u64, text: String) -> Self {
        let words = text.split_ascii_whitespace().count() as u32;

        Page {
            number,
            words,
            text,
        }
    }
}

#[cfg(test)]
mod structures_tests {
    use super::*;

    use rand::{seq::SliceRandom, thread_rng};

    #[test]
    fn empty_book() {
        let pages: BookPages = vec![];

        let book = Book::from(pages);

        assert_eq!("Empty", book.title);
        assert_eq!(0, book.num_pages());
    }

    #[test]
    fn one_empty_page_book() {
        let pages: BookPages = vec![("Book".to_string(), "".to_string())];

        let book = Book::from(pages);

        assert_eq!("Book", book.title);
        assert_eq!(1, book.num_pages());

        assert_eq!(1, book.pages[0].number);
        assert_eq!(0, book.pages[0].words);
        assert_eq!("", book.pages[0].text);
    }

    #[test]
    fn book_page_grouping() {
        const NUM_BOOKS: u32 = 3;
        const NUM_PAGES: u32 = 5;

        let mut data: BookPages = Vec::new();

        for bk in 0..NUM_BOOKS {
            for pg in 0..NUM_PAGES {
                let book = format!("Book {}", bk + 1);
                let page = format!(
                    "Page {} of Book {}. {}",
                    pg + 1,
                    bk + 1,
                    "X".repeat(((bk + 1) * (pg + 1) * 10) as usize)
                );

                data.push((book, page));
            }
        }

        let mut rng = thread_rng();
        let rand_data = &mut data;

        rand_data.shuffle(&mut rng);

        for (book, page) in data.clone() {
            // println!("Book: {}, page: {}", book, page);
        }

        let grouped = data.into_iter().into_group_map();

        assert_eq!(3, grouped.keys().len());

        for key in grouped.keys().sorted() {
            let book = grouped.get(key).unwrap();

            for page in book {
                assert!(page.contains(key));
            }

            println!("Book: {}, {:?}", key, book);
        }
    }
}
