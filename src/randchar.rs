use reqwest::blocking::Client;
use serde_json::Value;

pub fn fetch_random_character(){
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

    //lmfdb commands start here
}