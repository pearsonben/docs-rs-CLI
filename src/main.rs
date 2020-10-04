extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};


fn main () -> Result<(), reqwest::Error> {

    let site_url: &str = "https://docs.rs/";

    let res = get_request(&site_url);

    let text_body = res.text().unwrap();
    
    Ok(())

}

fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error>{
    let res = reqwest::get(url)?;
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