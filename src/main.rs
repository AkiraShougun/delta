use clap::{command, Arg, Command};
use reqwest::blocking::Client;
use serde_json::Value;

fn main(){
    let matcher = command!().subcommand(
        Command::new("print").about("Prints a statement.")
        .arg(
            Arg::new("statement")
        )
    ).subcommand(
        Command::new("randchar").about("Gets a random anime character")
    )
    .get_matches();

    // let print_val = matcher.subcommand_matches("print");
    // let randchar = matcher.subcommand_matches("randchar");
    // println!("{}",print_val.unwrap().get_one::<String>("statement").unwrap());

    if let Some(print_matches) = matcher.subcommand_matches("print") {
        let statement = print_matches.get_one::<String>("statement").unwrap();
        println!("{}", statement);
    }

    if let Some(_) = matcher.subcommand_matches("randchar") {
        fetch_random_character()
    }

}

fn fetch_random_character(){
    let http_client = Client::new();
    let http_result = http_client.get("https://api.jikan.moe/v4/random/characters").send();


    match http_result {
        Ok(response) => {
            let response_text = response.text().unwrap();
            let json: Value = serde_json::from_str(&response_text).unwrap();
            let name = json.get("data").unwrap().get("name");
            let about = json.get("data").unwrap().get("about");
            let url = json.get("data").unwrap().get("url");
            println!("The character is: {:} \n About: {:} \n More info in: {:}", serde_json::to_string_pretty(&name).unwrap(),serde_json::to_string_pretty(&about).unwrap(),serde_json::to_string_pretty(&url).unwrap());
        }
        Err(error) => println!("Error: {:#?}", error),
    }
}