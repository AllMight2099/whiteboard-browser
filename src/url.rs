use std::os::unix::net::{UnixListener, UnixStream};
use http::HeaderMap;
use reqwest::Url;
// use url::Url;

// TODO change this to &str later
#[derive(Debug, Clone)]
pub struct URL {
    // scheme: String, 
    // host: String, 
    // path: String,
    // uri: String,
    url: Url
}

impl URL{
    pub fn url(url: String) -> Option<Self> {
        // let scheme: String;
        // let host: String;
        // let path: String;
        let uri: String;

        let parsed = match Url::parse(&url) { 
            Ok (a) => {
                return Some(Self{url: a})
            }
            Err (_) => {
                return None
            }
        };

        // let url_str = url.as_str();
        // uri = url;

        // let Some((sch, rem_url)) = url.split_once("://");
        // if let Some((sch, rem_url)) = url_str.split_once("://") {
        //     println!("Scheme part: {}", rem_url);
        //     scheme = sch.to_string();
        //     if !rem_url.contains("/") {
        //         host = rem_url.to_string();
        //         path = "/".to_string();
        //     } else {
        //         let parts: Vec<&str> = rem_url.splitn(2, "/").collect();
        //         host = parts[0].to_string();
        //         path = format!("/{}", parts[1]);
        //     }

        // } else {
        //     // TODO: Handle error
        //     return None;
        // }

        // println!("Scheme: {}", scheme);
        
        // Some(Self {
        //     // scheme,
        //     // host,
        //     // path,
        //     uri
        // })
    }


    pub fn request(self) -> String {
        let request = reqwest::blocking::get(self.url.as_str());
        let mut headers: HeaderMap = HeaderMap::new();
        let mut body: String = String::new();
        match request {
            Ok(response) => {
                // println!("Response Status: {}", response.status());
                headers = response.headers().clone();

                // println!("Response Headers:\n{:#?}", response.headers());
                match response.text() {
                    Ok(text) => {
                        body = text;
                    },
                    Err(e) => {
                        println!("Failed to read response body: {}", e);
                    }
                }
                // println!("Response Body:\n{}", body);
            }, 
            Err(e) => {
                println!("Request failed: {}", e);
            }
        }


        for header in headers.iter() {
            assert!(header.0 != "transfer-encoding");
            assert!(header.0 != "content-encoding");
            // println!("{}: {:?}", header.0, header.1);
        }

        // socket = UnixStream::connect()
        return body;
    }

}




// impl URL {
//     fn new(&self, scheme: &str, host: &str, path: &str) -> Self {
//        self.scheme = 
// };

