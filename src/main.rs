mod url;

use url::URL;

fn main() {
    let example_url = "https://browser.engineering/examples/xiyouji.html";
    let url: URL = URL::url("https://example.com".to_string()).unwrap();

    load(url);

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