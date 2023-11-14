

pub mod indexer {
    use crate::crawl::crawler::CrawlResult;
    use scraper::{Html, Selector};
    

    pub struct ParsedResult {
        url: String,
        body: String,
        content: Vec<String>,
        description: String,
        images: Vec<Vec<u8>>,
        links: Vec<String>,
        title: String,
    }

    fn parse_content(document: &Html, content_selector: &Selector, mut content:Vec<String>) -> Vec<String> {
        for element in document.select(content_selector) {
            let text = element.text().collect::<String>();
            if !text.is_empty() {
                content.push(text);
            }
        }
        return content;
    }

    pub fn parse_crawl_results(crawl_results: Vec<CrawlResult>) -> Vec<ParsedResult> {
        let mut parsed_results: Vec<ParsedResult> = Vec::new();
        for crawl_result in crawl_results {
            parsed_results.push(parse_crawl_result(crawl_result));
        }
        return parsed_results;
    }

    fn parse_crawl_result(crawl_result: CrawlResult) -> ParsedResult {

        //initialise data to include in the parsed result to be passed to the index.

        //first initialise entries already in the crawled results
        let url = crawl_result.url;
        let body = crawl_result.body;
        let links = crawl_result.new_urls;

        //metadata entries
        let mut description = String::new();

        //non-metadata entries
        let mut content: Vec<String> = Vec::new();
        let mut images: Vec<Vec<u8>> = Vec::new();
        let mut title = String::new();
        let document = Html::parse_document(&body);


        //intialise selectors corresponding to above data.
        let title_selector = Selector::parse("title").unwrap();
        let p_selector = Selector::parse("p").unwrap();
        let h_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
        let metadata_selector = Selector::parse("meta").unwrap();
        let image_selector = Selector::parse("img").unwrap();


        //first select the title to include as part of the results
        if let Some(title_element) = document.select(&title_selector).next() {
            title = title_element.text().collect::<String>();
            println!("Title: {}", title);
        } else {
            println!("Title not found");
        }

        //parse first h elements and then p elements to gather all of the text contnet.
        content = parse_content(&document, &p_selector, parse_content(&document, &h_selector, content));


        /*
        Working with metadata: consists of two attributes (name) and (content), where name is covered by cases we want to include, such as description,
        author, keywords, etc.

        Hence need to parse all meta tags and then provide cases for all the above we would like to include.

        Alternatively use the following syntax inside of the selector: let description_selector = Selector::parse("meta[name='description']").unwrap();
        
        
         */
        for element in document.select(&metadata_selector) {
            if let Some(name) = element.value().attr("name") {
                match name {
                    "description" => {
                        if let Some(content) = element.value().attr("content") {
                            description = content.to_string();
                        }
                    },
                    _ => {
                        println!("Do nothing");
                    }
                }
            }
        }
         

       
        /*
        Working with images: need to convert string stored in the src attribute of the img tag as a U8 vector (vector of integer bytes), this can
        then be converted back to URI format which allows us to display the images in the search engine.
        
        
         */
        for element in document.select(&image_selector) {
            if let Some(src) = element.value().attr("src") {
                images.push(src.as_bytes().to_vec());
            }
            else{
                println!("Image not found");
            }
        }

        let parsed_result = ParsedResult {
            url,
            body,
            content,
            description,
            images,
            links,
            title,
        };

        parsed_result

        }
    

}