use clap::{command, Arg, Command};

mod fs;
mod randchar;

fn main(){
    let matcher = command!().subcommand(
        Command::new("print").about("Prints a statement.")
        .arg(
            Arg::new("statement")
        )   
    ).subcommand(
        Command::new("randchar").about("Gets a random anime character")
    ).subcommand(Command::new("fc")).about("Creates a txt file.")
    .get_matches();

    // let print_val = matcher.subcommand_matches("print");
    // let randchar = matcher.subcommand_matches("randchar");
    // println!("{}",print_val.unwrap().get_one::<String>("statement").unwrap());

    if let Some(print_matches) = matcher.subcommand_matches("print") {
        let statement = print_matches.get_one::<String>("statement").unwrap();
        println!("{}", statement);
    }

    if let Some(_) = matcher.subcommand_matches("randchar") {
        randchar::fetch_random_character();
    }

    if let Some(_) = matcher.subcommand_matches("fc"){
        fs::create_file();
    }

}

