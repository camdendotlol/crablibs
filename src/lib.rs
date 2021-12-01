use std::{error::Error, io::Write};
use std::{fs, io};
use textwrap::{Options, fill, termwidth};
use yansi::Paint;

pub struct Config {
  pub filename: String
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Missing template file name")
    };

    Ok(Config { filename })
  }
}

struct Clue {
  hint: String,
  answer: Option<String>
}

impl Clue {
  pub fn new(hint: String) -> Clue {
    Clue { hint, answer: None }
  }

  pub fn set_answer(&mut self, answer: String) -> () {
    self.answer = Some(answer);
  }
}

fn get_answer(clue: &String) -> Result<String, Box<dyn Error>> {
  print!("{}: ", Paint::rgb(229, 178, 255, clue));

  // Flushing stdout allows us to accept user input on the same line
  io::stdout().flush().unwrap();

  let mut answer = String::new();

  io::stdin()
    .read_line(&mut answer)
    .expect("Invalid answer");
  
  // Trim the newline from the input
  Ok(String::from(answer.trim()))
}

fn generate_final_text(text: String, clues: Vec<Clue>) -> String {
  let answers_to_print: Vec<String> = clues.into_iter().map(|clue| clue.answer.unwrap()).collect();

  let chars = text.chars();
  let mut new_chars: Vec<char> = vec![];
  let mut found_clue = false;
  let mut index = 0;

  for char in chars {
    if found_clue {
      // Don't add anything inside the brackets to the final output
      if char == '}' {
        found_clue = false;
      }
    } else {
      if char == '{' {
        found_clue = true;
        new_chars.extend(answers_to_print[index].chars());
        index = index + 1;
      } else {
        new_chars.push(char);
      }
    };
  };

  let final_text: String = new_chars.into_iter().collect();
  final_text
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let chars = contents.chars();

  let mut clues: Vec<Clue> = vec![];
  let mut found_clue = false;
  let mut new_clue = String::new();

  // Parse the file to look for clues
  for char in chars {
    if found_clue {
      if char == '}' {
        found_clue = false;
        clues.push(Clue::new(new_clue));
        new_clue = String::new();
      } else {
        new_clue.push(char)
      }
    }

    if char == '{' {
      found_clue = true
    };
  };

  // Set the answers for each clue
  for clue in &mut clues {
    let answer = get_answer(&clue.hint).unwrap();
    clue.set_answer(answer);
  }

  println!("{}", Paint::rgb(255, 228, 225, "\n--------------------\n"));

  let print_options = Options::new(termwidth());
  println!("{}", Paint::rgb(182, 252, 213, fill(&generate_final_text(contents, clues), print_options)));

  Ok(())
}