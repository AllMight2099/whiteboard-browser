mod url;
mod browser;

use url::URL;
use browser::Browser;

// const example_url: String = 

fn main() -> iced::Result {
    // let example_url = "https://example.com".to_string();
    let example_url: String = "https://browser.engineering/examples/xiyouji.html".to_string();

    let url: URL = URL::url(example_url).unwrap();
    
    // let browser:
    // load(url);
    browser::Browser::run(url)
    
    // iced::application(Browser::new, Browser::update, Browser::view).run()
    // println!("Hello, world!");

    // println!("url as a whole: {:#?}", url);
}

fn load(url: URL) {
    let content: String = url.request();
    show(content)
}

fn show(body: String) {
    println!();
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>'{
            in_tag = false;
        } else if !in_tag {
            print!("{}", c);
        }
    }
}
