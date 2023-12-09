pub mod index {
    use std::collections::HashMap;
    use crate::parse::parser::Document;
    

    //for now the document ID is just the URL of each page.
    //first we consider simple classical search methods, and expand later.
    pub enum Indexer {
        TermIndex (HashMap<Document, Vec<String>>),
        InvertedIndex (HashMap<Vec<String>, Document>)

    }

    impl Indexer {
        fn insert (&mut self, document:Document, terms: Vec<String>){
            match self {
                Indexer::TermIndex(map) => {
                    map.insert(document, terms);
                }
                Indexer::InvertedIndex(map) => {
                    map.insert(terms, document);
                }
            }
        }
    }

    //needs to take in 
    fn tokenise (content: String) -> Vec<String> {
        //remove all non alphabetic characters - text encoded as a stream of UTF encoded bytes (UTF-8)
        //gets all of thes chars, applies filter (keeping alphabetic characters), and then collects this stream back into a String

        let content: String = content.chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect();

        //delimit words by comma
        let words: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
        
        //remove stop words
        let stop_words = vec![
   "a", "an", "the", "and", "but", "in", "on", "of", "with", "is", "was", "by", "at", "to", "from", "which", "you", "it", "this", "that", "or", "be", "are", "been", "were", "would", "will", "shall", "should", "can", "could", "has", "have", "had", "not", "if", "else", "then", "for", "but", "or", "so", "no", "nor", "on", "at", "to", "from", "by", "in", "of", "up", "out", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should", "now"
];      

        words.into_iter().filter(|word| !stop_words.contains(&word.as_str())).collect()
    }

    fn fill_inverted_index(documents: Vec<Document>){
        println!("Nothing for now");
    }

    //here we will pass vector of documents to be inserted into both the term and inverted indices.
    fn fill_term_index (documents: Vec<Document>) {
        //make new instance of term index
        let mut term_index = Indexer::TermIndex(HashMap::new());
        for document in documents {
            //tokenise all content
            let pre_terms: Vec<Vec<String>> = document.content.iter().map(|content| tokenise(content.to_string())).collect();
            //collect into set of terms
            let terms = pre_terms.into_iter().flatten().collect();
            //now need to insert the document and terms into the term index.
            term_index.insert(document.clone(), terms);
        }

    pub fn create_index(){
        println!("Nothing here yet")
    }

    pub fn delete_index(){
        println!("Nothing here yet")
    }

    pub fn delete_all() {
        println!("Nothing here yet")
    }

       }
}
