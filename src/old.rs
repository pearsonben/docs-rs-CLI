extern crate reqwest;
extern crate scraper;
//use std::env;
use select::document::Document;
use select::predicate::{Class, Name};

fn main()-> Result<(), reqwest::Error> {

    // gets the CLI passed arguments
    //et cli_args: Vec<String> = env::args().collect();

    // default crates.io query url string
    let url = String::from("https://crates.io/search?q=html");
    // gets the single cli arg value
    //let search_term: &str = &cli_args[1];

    // gets the contents of "url"
    let res: reqwest::Response = get_request(&url).unwrap();

   // println!("{:#?}", res);

   // html body as a string
    let body = res.text().unwrap();

    let fragment:: 

/*
    for node in document.find(Class("_list_1hnt44")).take(10) {
        let crate_title = node.find(Class("_name_s6xett")).next().unwrap();
        println!("title: {}", crate_title.text());
        println!("Hello");

    }*/

    Ok(())
}
    
fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error>{
    let res = reqwest::get(url)?;

    Ok(res)
}