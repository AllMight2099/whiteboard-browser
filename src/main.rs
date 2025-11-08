mod url;

use url::URL;

fn main() {
    let url: URL = URL::url("https://example.com/path/example/yes".to_string()).unwrap();

    println!("Hello, world!");
    println!("url as a whole: {:#?}", url);
}
