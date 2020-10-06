extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use std::env;



#[derive(Debug)]
struct CrateListing {
    name: String,
    desc: String,
    date: String,
    url: String,
}

fn get_search_term() -> String {

    let cli_args: Vec<String> = env::args().collect();

    let search_term: &str = &cli_args[1];

    String::from(search_term)
}




fn main () -> Result<(), reqwest::Error> {

    // retrieves the CLI search term
    let search_term = get_search_term();

    // defines the default search address
    let mut site_url = String::from("https://docs.rs/releases/search?query=");
    // pushes the search term into the url
    site_url.push_str(&search_term);


    let mut res = get_request(&site_url).unwrap();

    let text_body = res.text().unwrap();
    let fragment = Html::parse_document(&text_body);



    let crate_names = Selector::parse(".name").unwrap();
    let crate_dates = Selector::parse(".date").unwrap();
    // dont ask why but .descriptions fucks the entire thing up. need to use wacky class name from below
    let crate_descriptions = Selector::parse(".pure-u-sm-14-24").unwrap();

    let mut crate_vector: Vec<CrateListing> = Vec::new();

    // iterate over every crate name 
    for crates in fragment.select(&crate_names){
        // gets the crate name string
        let crate_name = crates.text().collect::<Vec<_>>();
        //println!("crate_names length = {:#?}", crate_name.len());

        // gets rid of all the crap
        let crate_name = crate_name[0].replace(&['(', ')', ',', '\"', ';', ':', '\'', ' ', '\n'][..], "");

        // instantiates the crate struct
        let crate_listing = CrateListing {
            name: String::from(crate_name),
            desc: String::from(""),
            date: String::from(""),
            url: String::from("")
        };
        // pushes the struct onto the vector
        crate_vector.push(crate_listing);
    }

    let mut i = 0;
    for crates in fragment.select(&crate_dates){

        let crate_date = crates.text().collect::<Vec<_>>();
        let crate_date = crate_date[0].replace(&['(', ')', ',', '\"', ';', ':', '\'', ' ', '\n'][..], "");

        crate_vector[i].date = crate_date;
        i += 1;
    }

    let mut i = 0;
    for crates in fragment.select(&crate_descriptions){
        // gets the crate name string
        let crate_description = crates.text().collect::<Vec<_>>();
        let crate_description = crate_description[0].replace(&['(', ')', ',', '\"', ';', ':', '\'', '\n'][..], "");
        
        crate_vector[i].desc = String::from(crate_description.trim());
        i += 1;
        
    }



   
    crate_vector.iter().for_each(|it|{
        println!("{} | {} | {}\n", it.name, it.desc, it.date);
    
    });


    Ok(())



}

fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error>{
    let res = reqwest::get(url).unwrap();
    assert!(res.status().is_success());

    Ok(res)
}

