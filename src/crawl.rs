pub mod crawler {
    // Import statements go here
    use std::collections::BinaryHeap;

    #[derive(Debug, Eq, PartialEq)]
    struct UrlToVisit {
        url: String,
        crawl_depth: u32,
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

    // Code implementation goes here
    pub fn crawl(seed_urls: &Vec<Vec<String>>) {
        //end crawl when tuple popped with depth = 10.
        let mut url_queue: BinaryHeap<UrlToVisit> = BinaryHeap::new();

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