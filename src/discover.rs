pub mod discoverer {
    // Import statements go here
    use serde::Deserialize;
    use serde_json::from_str;
    use std::fs::File;
    use std::io::Read;

    // Code implementation goes here
    #[derive(Debug, Deserialize)]
    struct EduDomain {
        name: String,
        domains: Vec<String>,
        web_pages: Vec<String>,
        country: String,
        alpha_two_code: String,
        state_province: Option<String>,
    }

    //should return error in the case of a problem with IO operation, otherwise should just return string contents.
    fn read_domains_json() -> Result<String, std::io::Error> {
        let mut file = File::open("domains.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn get_domains_and_webpages() -> (Vec<Vec<String>> , Vec<Vec<String>>) {
        let mut edu_domains: Vec<EduDomain> = Vec::new();
        let mut domains: Vec<Vec<String>> = Vec::new();
        let mut seed_urls: Vec<Vec<String>> = Vec::new();

        match read_domains_json() {
            Ok(contents) => {
                edu_domains = from_str(&contents).unwrap();
                domains = Vec::new();
                seed_urls = Vec::new();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        for edu_domain in edu_domains {
            let domains_vec = edu_domain.domains.clone();
            let web_pages_vec = edu_domain.web_pages.clone();

            domains.extend(vec![domains_vec]);
            seed_urls.extend(vec![web_pages_vec]);
        }
        
        (seed_urls, domains)
    }


}