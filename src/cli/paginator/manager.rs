use crate::cli::input;

pub struct ManagerAPI {
    current_page: usize,
    pages: Vec<String>,
}

impl ManagerAPI {
    pub fn add_page(&mut self, page: &str) {
        self.pages.push(String::from(page));
    }
    fn ask_for_navigation(&self) {
        let where_to_go = input::from_user::generic(
            "Where do you want to navigate for? Type [n]ext page, [p]revious page, page # or [s]kip.",
        );

        match &where_to_go.to_lowercase()[..] {
            "n" => self.next(),
            "p" => self.previous(),
            "s" => println!("Skipped!"),
            _ => {
                let page: usize = where_to_go.to_string().parse().unwrap();
                println!("Considering page as a number and going to page {}", page);
                self.go_to_page(page);
            }
        }
    }

    pub fn get_page_by_nuber(&self, page_number: usize) -> &str {
        &self.pages[page_number][..]
    }

    pub fn get_pages(&self) -> &Vec<String> {
        &self.pages
    }

    pub fn go_to_page(&self, page_number: usize) {
        let page = self.pages.get(page_number - 1);

        match &page {
            Some(page) => {
                println!("{page}");
                self.show_remaing_pages();
            }
            None => self.ask_for_navigation(),
        }
    }

    pub fn next(&self) {
        if self.current_page == self.pages.len() {
            println!("Already on the last page!");
            self.ask_for_navigation();
        } else if self.current_page < self.pages.len() {
            self.go_to_page(self.current_page + 1);
        }
    }
    pub fn previous(&self) {
        if self.current_page == 1 {
            println!("Already on the first page!");
            self.ask_for_navigation();
        } else if self.current_page > 1 {
            self.go_to_page(self.current_page - 1);
        }
    }
    pub fn remove_page(&self, page_number: usize) {
        todo!("Find how to remove a specific item from the vector");
    }

    fn show_remaing_pages(&self) {
        println!("Page {} of {}", self.current_page, self.pages.len());
        self.ask_for_navigation();
    }

    fn start(&self) {
        for page in &self.pages {
            println!("{}", page);
        }

        if self.pages.len() > 1 {
            self.show_remaing_pages();
        }
    }
}

pub fn new(whole_formatted_text: &str, rows: u8, columns: usize) -> ManagerAPI {
    let manager_api = ManagerAPI {
        current_page: 1,
        pages: parse_text_to_pages(String::from(whole_formatted_text), rows, columns),
    };

    manager_api.start();
    manager_api
}

pub fn from_vec(pages: Vec<&str>) -> ManagerAPI {
    let mut pages_as_string: Vec<String> = Vec::new();
    for page in pages {
        pages_as_string.push(String::from(page));
    }

    let manager_api = ManagerAPI {
        current_page: 1,
        pages: pages_as_string,
    };

    manager_api.start();
    manager_api
}

pub fn from_raw_text(whole_text: &str, rows: u8, columns: usize) -> ManagerAPI {
    let manager_api = ManagerAPI {
        current_page: 1,
        pages: parse_text_to_pages(String::from(whole_text), rows, columns),
    };

    manager_api.start();
    manager_api
}

fn parse_text_to_pages(whole_text: String, rows: u8, columns: usize) -> Vec<String> {
    let mut row = String::new();
    let mut rows: Vec<String> = Vec::new();

    for (index, character) in whole_text
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
    {
        // the idea here is looping trough each characterand
        // - if it's index is below the columns ammount, then push it to the row, else
        // - check if the current character is empty,
        //  - if so, push the whole line to the rows vector;
        //      - if not, then check what is closer the end of the word or moving it to the
        //      next line. May be the case of this iteration be responsible for adding line
        //      breakes to the row and then the rows may be build from the line breakes split.
        if index == columns / index {
            // meaning that the caracter is now on the horizontal edge, so it would go to the next
            // line, but if it istn't a space, we need to get the whole word or let it to the next
            // line.
            if character == ' ' {}
        }
    }

    vec![String::from("")]
}
