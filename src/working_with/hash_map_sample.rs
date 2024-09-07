use std::collections::HashMap;

use crate::cli::output;
use crate::cli::paginator;

const SECTION_DESCRIPTION: &str = "\n
    In this section you'll be prompted to add a bunch of countries names which are\n
    participating in the Olympics. You can add only the country names of as many\n
    countries, then you can add the medals for a specific country some time during\n
    the countries entries. To do this just type in \"add_medals\" and you'll be\n
    able to choose one of the countries that you have already added by it's number\n
    for example, you'll be show a list like this:\n
    \'1 - First country you added\'\n
    \'2 - Second country you added\'\n
    \'...\'\n\n
    Then if you want to add medals to the second country you add just type\n
    \'2 gold=1 silver=2 bronze=3\'. It's important to follow exactly this pattern\n
    to have the medals properly added to that country. So the patter is first the\n
    number from the list that represents the country added, then the medal, the\n
    equals sign and then the amount of medals. The medals order doesn't matter\n
    meaning that you can add only gold or bronze or even in a different order\n
    like silver=2 gold=3 bronze=4. The most important is first the number then the\n
    medals with equals sign and each medal separated by a space.\n\n
    To go back adding new countries, just type \'add_countries\' and go for it, but\n
    there is a way easier to add both together which is while adding a country, you\n
    can type \'name=\"country name\" gold=2 silver=3 bronze=4\' and it will register\n
    the country with that amount of medals. If that country was already added,\n
    you'll be prompted if you want to proceed with the addition, since the country\n
    is duplicated, if so, all medals will be added, but don't worry if you pressed\n
    wrong key or added when you shouldn't, because you can also update a country\n
    by typing, guess what, \"update country\". You'll then receive a list of\n
    registered countries, pick one by its number and follow the instructions or\n
    use the same pattern defined when adding medals, by choosing a country number.\n
    If you know already the country number, you can type all at once like this\n
    \'update 2 | name=\"country name\" gold=1 silver=1 bronze=1\', but the difference\n
    is that if you update the country medals, it won't add it'll replace the numbers.\n\n
    Finally you can always use the command \'show\' to list the registered ones\n
    and \'delete\', to pick one to delete. If you know the country number, than\n
    just type \'delete 2\' (where 2 is the country number for example). When the list\n
    is done type \'done\' and you be printed with the medal standings board. Last but\n
    not least, if you forget any for this commands and these tips, you can type\n
    \'help\' and a list with the commands will be printed.
    ";

pub fn run() {
    paginator::manager::new(SECTION_DESCRIPTION, 20, 150);
    // TODO: add pagination by creating a linked list os just a Vec<String> that holds just a slice
    // of text to be rendered and at the end expects to "n" to go to the next page and "p" to go
    // back to the previous and any other to skip the whole text. Or even it should be interesting
    // showing like 1 of 5 and just press the number to go to the given page.
    output::show::title("Welcome to Hash Map Sample", SECTION_DESCRIPTION);
    // TODO: Add a input on cli::input module to make yes / no questions like this
    println!("Are you ready to start?");

    // TODO: Ideas:
    // - to make it easier to process the commands in the while true loop is by the pattern matching with an enum to
    //   list the commands and the ones that don't match may be new entries or commands with entries if there is any
    //   = sign in the string
    // - add a command to save the entries to a file
    // - ask if the person wants to save the entries to a file when done command is called and has
    //   entries not saved
    // - add a command to save the medal standinds board to a file

    let mut _medals_map: HashMap<&str, u32> = HashMap::new();
    let mut _country_map: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut _countries_map: HashMap<&str, HashMap<&str, HashMap<&str, u32>>> = HashMap::new();
}
