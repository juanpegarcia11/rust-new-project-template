/* Use wikipedia crate to fetch pages

 - Process page content
 - Collect timing
 - Concurrent page processing
*/

// Import libraries
use rayon::prelude::*;
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;

// Struct for Processed page
struct ProcessedPage {
    title: String,
    content: String
}

// Nine top NBA players
const PAGES: [&str; 9] = [
    "Michael Jordan",
    "Kobe Bryant",
    "Lebron James",
    "Stephen Curry",
    "Kevin Garnett",
    "Dwight Howard",
    "Russell Westbrook",
    "Kyrie Irving",
    "Magic Johnson",
];

fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page.get_title().unwrap();
    let content = page.get_content().unwrap();
    ProcessedPage {
        title,
        content
    }
}

//time how long it takes to process pages
fn main() {
    let start = std::time::Instant::now();
    let wiki = Wikipedia::<Client>::default();
    let pages : Vec<_> = PAGES.par_iter().map(|&p| wiki.page_from_title(p.to_string())).collect();
    let processed_pages: Vec <ProcessedPage> = pages.par_iter().map(process_page).collect();
    for page in processed_pages {
        let _start_page = std::time::Instant::now();
        println!("Title: {}", page.title.as_str());
        println!("Content: {}", page.content.as_str());
        let elapsed_page = start.elapsed();
        println!("Elapsed_Page: {:.2?}", elapsed_page);
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
