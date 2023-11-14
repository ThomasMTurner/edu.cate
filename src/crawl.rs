
pub mod crawler {
    use std::collections::BinaryHeap;
    use reqwest::get;
    use scraper::{Html, Selector};
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct UrlToVisit {
        url: String,
        crawl_depth: u32,
    }

    #[derive(Clone, Debug)]
    pub struct CrawlResult {
        pub url: String,
        pub new_urls: Vec<String>,
        pub body: String,
    }

    //implement ordering for priority queue which is based on lowest crawl depth first
    impl PartialOrd for UrlToVisit {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.crawl_depth.cmp(&other.crawl_depth).reverse())
        }
    }

    //implement the full ordering with error handling layer above to catch errors in comparing tuples.
    impl Ord for UrlToVisit {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.partial_cmp(other) {
                Some(ord) => ord,
                None => std::cmp::Ordering::Equal,
            }
        }
    }

    async fn get_crawl_result(url: &str) -> Result<CrawlResult, reqwest::Error> {
        let mut new_urls: Vec<String> = Vec::new();
        let response = get(url).await?;
        let body = response.text().await?;
        let fragment = Html::parse_document(&body);
        let url_selector = Selector::parse("a").unwrap();
    
        for element in fragment.select(&url_selector) {
            if let Some(url) = element.value().attr("href") {
                new_urls.push(url.to_string());
            } else {
                // Do nothing
                println!("No new URLs found for this one.");
            }
        }
        
        println!("This is the body of the page: {:?}", body);

        let crawl_result = CrawlResult {
            url: url.to_string(),
            new_urls,
            body,
        };
    
        Ok(crawl_result)
    }
    

    pub async fn crawl(seed_urls: &mut Vec<String>, max_depth: u32) -> Vec<CrawlResult> {
        println!("Starting crawl...");
        println!("Seed URLs: {:?}", seed_urls);
        //initialise visited to not re-process
        let mut visited: Vec<UrlToVisit> = Vec::new();
        //initialise priority queue to crawl URL's
        let mut url_queue: BinaryHeap<UrlToVisit> = BinaryHeap::new();
        let mut results: Vec<CrawlResult> = Vec::new();

        //add seed URL's to be processed first in the queue.
        for seed_url in seed_urls {
            url_queue.push(UrlToVisit {
                url: seed_url.clone(),
                crawl_depth: 0,
            })
        }

        //while the url_queue is not empty and depth is less than 10
        while !(url_queue.is_empty()) && (url_queue.peek().unwrap().crawl_depth < max_depth) {

            //pop the next url from the queue
            let next_url = match url_queue.pop() {
                Some(url) => url,
                None => {
                    continue;
                }
            };
            
            println!("Next URL: {:?}", next_url);

            //if not visited
            if !(visited.contains(&next_url)) {
                //increment depth for new crawl result
                let new_depth: u32 = next_url.crawl_depth + 1;
                //get the body response from reqwest::get.
                let crawl_result = get_crawl_result(&next_url.url).await;
                match crawl_result {
                    Ok(result) => {
                        let res = result.clone();
                        results.push(result);
                        for url in res.new_urls {
                            url_queue.push(UrlToVisit {
                                url,
                                crawl_depth: new_depth,
                            })
                        }
                    }
                    Err(_) => {
                        //do nothing
                    }
                }


                //add the URL now to visited
                visited.push(next_url);
            
            }
        


        }

        return results;

    }
}


/*
Crawling algorithm version 1:


This is the iterative (non-recursive) version of the crawling algorithm
    (1) maintain priority queue of seed URL's and visited
    (2) get URL from the queue
    (3) Check if URL has already been visited, if so, remove from queue 
    (4) If not, get the URL, add to visited, parse URL for different content (can leave step omitted)
    (5) Get the URL's from within the parsed HTML, and enqueue these.


*/