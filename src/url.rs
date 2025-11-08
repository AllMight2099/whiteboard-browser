use std::os::unix::net::{UnixListener, UnixStream};

// TODO change this to &str later
#[derive(Debug)]
pub struct URL {
    scheme: String, 
    host: String, 
    path: String,
}

impl URL{
    pub fn url(url: String) -> Option<Self> {
        let scheme: String;
        let host: String;
        let path: String;

        let url_str = url.as_str();

        // let Some((sch, rem_url)) = url.split_once("://");
        if let Some((sch, rem_url)) = url_str.split_once("://") {
            println!("Scheme part: {}", rem_url);
            scheme = sch.to_string();
            if !rem_url.contains("/") {
                host = rem_url.to_string();
                path = "/".to_string();
            } else {
                let parts: Vec<&str> = rem_url.splitn(2, "/").collect();
                host = parts[0].to_string();
                path = format!("/{}", parts[1]);
            }

        } else {
            // TODO: Handle error
            return None;
        }

        println!("Scheme: {}", scheme);
        
        Some(Self {
            scheme,
            host,
            path,
        })
    }


    // pub fn request() {
    //     socket = UnixStream::connect()
    //     return;
    // }

}




// impl URL {
//     fn new(&self, scheme: &str, host: &str, path: &str) -> Self {
//        self.scheme = 
// };

