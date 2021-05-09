// Just a simple english dictionary reader for you.

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use colored::*;
use difflib::get_close_matches as gcm;

#[allow(dead_code)]
fn file_content(file_name: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_name)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

#[allow(dead_code)]
fn file_buffer(file_name: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}

fn parse_newlines(string: &str) -> String {
    string.replace("\\n", "\n")
}

fn print_definition(some_word: Option<&str>, definition: &str) {
    let definition = parse_newlines(definition);

    if let Some(word) = some_word {
        println!("{} -\n\n{}", word.bold().green(), definition.italic());
    } else {
        println!("{}", definition.italic());
    }
}

fn main() {
    const PARSE_FAIL_MSG: &str =
        "Are you sure the provided file is a valid dictionary file? Maybe try removing the entire line or reinstall the file.";

    const DICT_ENV_ERR_MSG: &str =
        "Dictionary file path is not defined in your env. Try executing:";

    const DICT_ENV_ERR_TRY: &str =
        "export DICTIONARY_FILE='put absolute path of the dictionary file here'";

    const FILE_ERR_MSG: &str = "The dictionary file does not exist.";

    const FILE_ERR_TRY: &str =
        "Are you sure that the path you set is a valid absolute path to the dictionary file?";

    const HELP: &str = "Get the definition of any english word offline. Thanks for using :)";

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() == 1 || args[1] == "--help" {
        println!(
            "Usage:\n  {} {}\nHelp:\n  {}",
            args[0].bold(),
            "<word>".bold(),
            HELP.blue()
        );
        exit(0);
    }

    let arg_word = args[1].trim().to_lowercase();

    let file_path = env::var("DICTIONARY_FILE").unwrap_or_else(|_| {
        println!("{}\n{}", DICT_ENV_ERR_MSG.red(), DICT_ENV_ERR_TRY.yellow());
        exit(2);
    });

    let data = file_content(&file_path).unwrap_or_else(|_| {
        println!("{}\n{}", FILE_ERR_MSG.red(), FILE_ERR_TRY.yellow());
        exit(2);
    });

    let mut dict: HashMap<&str, &str> = HashMap::new();
    let mut dict_words: Vec<&str> = Vec::new();

    for (n, line) in data.lines().enumerate() {
        let mut line_split = line.splitn(2, ":");

        let word = line_split
            .next()
            .unwrap_or_else(|| {
                println!(
                    "Failed to parse the word at line {}. The line:\n{}\n{}",
                    n.to_string().blue(),
                    line.red(),
                    PARSE_FAIL_MSG.yellow()
                );
                exit(1);
            })
            .trim();

        let definition = line_split
            .next()
            .unwrap_or_else(|| {
                println!(
                    "Failed to parse the definition at line {}. The line:\n{}\n{}",
                    n.to_string().blue(),
                    line.red(),
                    PARSE_FAIL_MSG.yellow()
                );
                exit(1);
            })
            .trim();

        if word == arg_word {
            print_definition(None, definition);
            exit(0);
        }

        dict.insert(word, definition);
        dict_words.push(word);
    }

    let close_matches = &gcm(&arg_word, dict_words, 10, 0.6);

    if close_matches.len() == 0 {
        println!("{}", "No match found.".red());
        exit(1);
    }

    println!(
        "Did you mean: {} ?\n",
        close_matches.join(", ").yellow().italic()
    );
    println!("{}", "Giving definition of the closest match..\n");

    print_definition(
        Some(&close_matches[0]),
        dict.get(&close_matches[0]).unwrap(),
    );

    exit(0);
}
