use clap::{command, Arg, Command};

fn main(){
    let matcher = command!().subcommand(
        Command::new("print").about("Prints a statement.")
        .arg(
            Arg::new("statement")
        )
    ).get_matches();

    let print_val = matcher.subcommand_matches("print");
    println!("{}",print_val.unwrap().get_one::<String>("statement").unwrap())
}