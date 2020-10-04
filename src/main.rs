extern crate reqwest;
extern crate scraper;

mod structures;
use std::string;
use scraper::{Html, Selector};

#[derive(Debug)]
struct Crate_Listing {
    name: String,
    desc: String,
    date: String,
    url: String,
}


fn main () -> Result<(), reqwest::Error> {

    let site_url: &str = "https://docs.rs/releases/search?query=html";
    let crate_url =  String::from("https://docs.rs/");

    let mut res = get_request(&site_url).unwrap();

    let text_body = res.text().unwrap();
    let fragment = Html::parse_document(&text_body);

    let crate_names = Selector::parse(".name").unwrap();
    let crate_descriptions = Selector::parse(".description").unwrap();
    let crate_dates = Selector::parse(".date").unwrap();

    let mut crate_vector: Vec<Crate_Listing> = Vec::new();


    // iterate over every crate name 
    for crates in fragment.select(&crate_names){
        // gets the crate name string
        let mut crate_name = crates.text().collect::<Vec<&str>>();

        // instantiates the crate struct
        let crate_listing = Crate_Listing {
            name: String::from(crate_name[0]),
            desc: String::from(""),
            date: String::from(""),
            url: String::from("")
        };
        // pushes the struct onto the vector
        crate_vector.push(crate_listing);

        // gets rid of all the crap
        let crate_name = crate_name[0].replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', ' ', '\n'][..], "");
        println!("{}", String::from(crate_name));
    
    }
    

    Ok(())



}

fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error>{
    let res = reqwest::get(url).unwrap();
    Ok(res)
}

/*fn main(){

    // env::args().collect() reads the command line arguments. 
    // e.g "docs_cli_browser [collect() reads text in brackets]"
    // cli_args type needs to be defined, as collect() return value cannot be inferred
    let cli_args: Vec<String> = env::args().collect();

    let search_term: &str = &cli_args[1];


    // :? is the debugging formatter. it will print every item in the collection
    println!("Search term was: {:?}", search_term);

}*/