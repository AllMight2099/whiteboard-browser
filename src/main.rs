mod url;
mod tab;
mod view;

use url::URL;
// use tab::Tab;
use iced::{Element, Length, Task, message, widget::container};

use crate::view::TabView;

// const example_url: String 

#[derive(Default)]
pub struct App{
    tabs: Vec<tab::Tab>,

    // current_tab: tabIDs,
    // title: String,
}

#[derive(Debug, Clone)]
enum Event {
    CloseTab, // need to take in tab ID here 
    CreateTab, // new tab ID?
    Tab(tab::Message), // need to take in tab ID here
}

impl App {
    fn new() -> (Self, Task<Event>) {
        
        let tab = tab::Tab::new(
            URL::url("https://browser.engineering/examples/xiyouji.html".to_string()).unwrap()
        );

        (App{
            tabs: vec![tab],
            // title: "New Window".to_string(),
        }, 
    Task::none())
    }


    fn update(&mut self, event: Event) -> Task<Event> {
        println!("In update function");
        match event{
            Event::CloseTab => {
                return Task::none()
            },
            Event::CreateTab => {
                return Task::none()
            },
            Event::Tab(_) => {
                // self.tabs[0].update(message);
                return Task::none()
            }
        };
    }


    fn title(&self) -> String {
        let title: String = "Whiteboard Browser".to_string();
        println!("In title function");

        return title;
    }


    fn view(&'_ self) -> Element<'_, Event> {
        // container
        let tab = &self.tabs[0];
        container(TabView::show(tab).map(Event::Tab)).width(Length::Fill).height(Length::Fill).into()
    }

}

fn main() -> iced::Result {
    // let example_url = "https://example.com".to_string();
    let example_url: String = "https://browser.engineering/examples/xiyouji.html".to_string();

    let url: URL = URL::url(example_url).unwrap();
    
    // let browser:
    // load(url);
    // browser::Browser::run(url)

    
    // iced::application(Browser::new, Browser::update, Browser::view).run()
    // println!("Hello, world!");

    // println!("url as a whole: {:#?}", url);

    iced::application(App::new, App::update, App::view).title(App::title).run()
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
