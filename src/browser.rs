use iced::{Element, Settings};
// use iced::{executor, Application, Theme};
use iced::widget::{scrollable, text};
use crate::url::URL;

#[derive(Default)]
pub struct Browser{
    displayList: Vec<String>,
    text: String,
}

#[derive(Debug, Clone)]
pub enum Message {

}


impl Browser {
    fn new(url: URL) -> Self {
        let text: String = Self::load(url);
        Self{
            displayList: Vec::new(),
            text,
        }
    }

    pub fn run(url: URL) -> iced::Result {
        let boot = move || Browser::new(url.clone());
        iced::application(boot, Browser::update, Browser::view).settings(Settings::default()).run()
    }

    fn update(&mut self, message: Message) {
        println!("In update function");
        match message{};
        // Command::none()
    }

    fn title(&self) -> String {
        let title: String = "Whiteboard Browser".to_string();
        println!("In title function");
        return title;
    }

    fn view(&self) -> Element<'_, Message>  {
    // fn view(&self){
        println!("In view function");
        // let text = Self::load(self.text.clone());
        println!("Text to display: {}", self.text);
        scrollable(text(self.text.clone())).into()
    }

    fn load(url: URL) -> String{
        let content: String = url.request();
        let text: String = lex(content);
        // show(content)
        return text;
    }
}

fn layout() {

}

fn lex(body: String)  -> String {
    let mut text: String = "".to_string();
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>'{
            in_tag = false;
        } else if !in_tag {
            text.push(c);
            // print!("{}", c);
        }
    }

    text
}
