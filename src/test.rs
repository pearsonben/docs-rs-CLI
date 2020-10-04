extern crate reqwest;
extern crate scraper;

// importation syntax
use scraper::{Html, Selector};

fn main() {
    hn_headlines("https://docs.rs/releases/search?query=html");
}

fn hn_headlines(url: &str) {

   let mut resp = reqwest::get(url).unwrap();
   assert!(resp.status().is_success());

   let body = resp.text().unwrap();
   // parses string of HTML as a document
   let fragment = Html::parse_document(&body);
   // parses based on a CSS selector
   let stories = Selector::parse(".name").unwrap();

   // iterate over elements matching our selector
   for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }
}