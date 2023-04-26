#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub pages: Pages,
}

type Books = Vec<Book>;

#[derive(Debug, Clone)]
pub struct Page {
    pub number: u64,
    pub words: u32,
    pub text: String,
}

type Pages = Vec<Page>;

type BookPage = (String, String);
type BookPages = Vec<BookPage>;

impl Book {
    fn new(title: String, pages: Pages) -> Self {
        Book { title, pages }
    }

    fn num_pages(&self) -> u32 {
        self.pages.len() as u32
    }
}

#[derive(Default)]
pub struct BookBuilder {
    entries: BookPages,
}

impl BookBuilder {
    fn new(entries: BookPages) -> BookBuilder {
        BookBuilder { entries }
    }

    fn build(self) -> Books {
        let mut map: HashMap<String, Pages> = HashMap::new();

        for (book, page) in self.entries {
            if !map.contains_key(&book) {
                map.insert(book.clone(), Vec::new());
            }

            if let Some(book_pages) = map.get_mut(&book) {
                let page_number = (book_pages.len() + 1) as u64;

                book_pages.push(Page::new(page_number, page));
            }
        }

        let mut books: Books = Vec::new();

        let book_titles = map.keys().cloned().collect::<Vec<_>>();

        for title in book_titles {
            let pages = map.remove(&title).unwrap();
            books.push(Book::new(title, pages));
        }

        books
    }
}

use std::collections::HashMap;

impl From<BookPages> for Book {
    fn from(book_pages: BookPages) -> Book {
        let mut map: HashMap<String, Pages> = HashMap::new();

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
            let book = map.keys().next().unwrap();
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

    use itertools::Itertools;

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

    const NUM_BOOKS: u32 = 3;
    const NUM_PAGES: u32 = 5;

    fn make_books(nbooks: u32, npages: u32) -> BookPages {
        let mut data: BookPages = Vec::new();

        for pg in 0..npages + nbooks {
            for bk in 0..nbooks {
                if (bk + npages) > pg {
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
        }

        data
    }

    #[test]
    fn book_page_grouping() {
        let data = make_books(NUM_BOOKS, NUM_PAGES);

        let grouped = data.into_iter().into_group_map();

        assert_eq!(3, grouped.keys().len());

        for key in grouped.keys().sorted() {
            let book = grouped.get(key).unwrap();

            for page in book {
                assert!(page.contains(key));
            }

            println!("Book: '{}' has {} pages", key, book.len());
        }
    }

    #[test]
    fn builder_builds() {
        let num_books = NUM_BOOKS;
        let num_pages = NUM_PAGES;

        let data = make_books(num_books, num_pages);

        let builder = BookBuilder::new(data);

        let books = builder.build();

        assert_eq!(num_books, books.len() as u32);

        for book in books {
            let book_num = book.title.split_ascii_whitespace().collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();

            assert!(book.title.contains("Book "));
            assert_eq!(book_num + num_pages - 1, book.num_pages());

            println!("Book: '{}' has {} pages", book.title, book.num_pages());
        }
    }

    #[test]
    fn builder_builds_with_no_books() {
        let data: BookPages = Vec::new();

        let builder = BookBuilder::new(data);

        let books = builder.build();

        assert_eq!(0, books.len() as u32);
    }

    #[test]
    fn builder_builds_with_default() {
        let builder = BookBuilder::default();

        let books = builder.build();

        assert_eq!(0, books.len() as u32);
    }
}
