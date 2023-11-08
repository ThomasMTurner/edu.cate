mod discover;
mod crawl;
use discover::discoverer::get_domains_and_webpages;
use crawl::crawler::crawl;


/*
(1) obtain domains and webpages from JSON file
(2) call GET to all of the domains to check if any webpages exists not covered already by the domains list 
(3) Once obtained all of the webpages we can move over to the crawl process - that is, pass web pages to the crawler.


*/

fn main() {
    let seed_urls;
    (seed_urls, _) = get_domains_and_webpages();
    crawl(&seed_urls);
    println!("Seed URL's: {:?}", seed_urls);
}
