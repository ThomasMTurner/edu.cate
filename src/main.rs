mod discover;
mod index;
mod crawl;
mod parse;

use discover::discoverer::get_domains_and_webpages;
use crawl::crawler::{crawl, CrawlResult};
use tokio::runtime;
use parse::parser::parse_crawl_results;

/*
(1) obtain domains and webpages from JSON file
(2) call GET to all of the domains to check if any webpages exists not covered already by the domains list 
(3) Once obtained all of the webpages we can move over to the crawl process - that is, pass web pages to the crawler.
*/

async fn get_crawled (seed_urls: Vec<String>, max_depth: u32) -> Vec<CrawlResult> {
    let mut seed_urls = seed_urls;
    let results = crawl(&mut seed_urls, max_depth).await;
    return results;
}

fn main() {
    let seed_urls;
    //first obtain the initial URL's to use in our crawl process
    (seed_urls, _) = get_domains_and_webpages();
    //select the first 3 seed URL's due to current slow processing
    let temp_seed_urls = &seed_urls[0..3];
    let seed_urls = temp_seed_urls.to_vec();
    
    //create a runtime to allow our async get_crawled (which is async due to calls to GET using the Reqwest crate) to return results while waiting for other code.
    let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();


    //provide a block on for the async get_crawled so that when it processes each request it adds these to results.
    let results = rt.block_on(get_crawled(seed_urls, 1));

    //to show all of the results which have been HTML parsed.
    let parsed_results = parse_crawl_results(results.clone());

    //Now obtained all of the results, want to parse HTML.
    println!("Parsed results: {:?}", parsed_results);
 }
