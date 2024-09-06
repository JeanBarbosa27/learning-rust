// TODO:
// - It should be interesting refactoring this into a trait, to be able to add a back field to be received as a
// function and also receive the items as a vector of hash maps like HashMap<String, Trait> where
// the string is the menu item title and the Fn is the reference to run when that option is
// choosen.
//

pub struct Menu {
    title: String,
    items: Vec<String>,
}

pub fn new(title: &str, items: Vec<String>) -> Menu {
    Menu {
        title: String::from(title),
        items,
    }
}

pub fn show(menu: Menu) {
    let mut items_to_print: Vec<String> = Vec::new();
    let mut bigger_item_length = 0;

    for (index, item) in menu.items.iter().enumerate() {
        let item_to_print = format!("{}: {}", index + 1, item);
        items_to_print.push(item_to_print.to_owned());

        let current_item_length = item_to_print.chars().count();
        if current_item_length > bigger_item_length {
            bigger_item_length = current_item_length;
        }
    }

    let decorated_line = "-".repeat(bigger_item_length);
    println!("{}", menu.title);
    println!("{decorated_line}");
    items_to_print.iter().for_each(|item| println!("{item}"));
    println!("{decorated_line}\n");
}
