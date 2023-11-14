mod discover;
mod index;
mod crawl;

use discover::discoverer::get_domains_and_webpages;
use crawl::crawler::{crawl, CrawlResult};
use tokio::runtime;
//use index::indexer::{}; 

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
    (seed_urls, _) = get_domains_and_webpages();
 
    let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();

    let results = rt.block_on(get_crawled(seed_urls, 10));

    //Now obtained all of the results, want to parse HTML.
 
    println!("Search results: {:?}", results);
 }
