use iced::{Alignment, Color, Element, Font, Pixels, Settings};
// use iced::{executor, Application, Theme};
use iced::widget::{scrollable, text, canvas};
use iced::mouse;
use iced::{Point, Rectangle, Renderer, Theme};
use crate::url::URL;

#[derive(Default, Clone, Debug)]
pub struct Browser{
    displayList: Vec<String>,
    text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    // ScrollDown, 
}

const HSTEP: i32 = 13;
const VSTEP: i32 = 18;


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

        canvas(Browser{ displayList: self.displayList.clone(), text: self.text.clone()}).into()
        // scrollable(text(self.text.clone())).into()
    }

    fn load(url: URL) -> String{
        let content: String = url.request();
        let text: String = lex(content);
        // show(content)
        return text;
    }
}

impl<Message> canvas::Program<Message> for Browser {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        println!("Insiode draw maybe?");

        // We create a `Path` representing a simple circle
        // let circle = canvas::Path::circle(frame.center(), self.radius);

        let dpList  = self.displayList.clone();
        let mut y = 0.0;
        for text in dpList {
            frame.fill_text(canvas::Text{
                content: text,
                position: Point{x:0.0, y:0.0},
                max_width: bounds.width,
                color: Color::BLACK,
                size: Pixels(16.0),
                line_height: text::LineHeight::Relative(1.2),
                font: Font::DEFAULT,
                align_x: text::Alignment::Default,
                align_y: iced::alignment::Vertical::Top,
                shaping:  text::Shaping::Basic,
            });
        }
        // frame.fill_text(text);


        // println!("{:#?}", frame);

        // And fill it with some color
        // frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

fn layout() {
    let disp_list: Vec<(Point, String)> = Vec::new();

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
