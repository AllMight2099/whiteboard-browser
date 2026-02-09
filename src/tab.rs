use iced::widget::canvas::Cache;
use iced::{Alignment, Color, Element, Font, Pixels, Settings};
// use iced::{executor, Application, Theme};
use iced::widget::{scrollable, text, canvas};
use iced::mouse;
use iced::{Point, Rectangle, Renderer, Theme};
use crate::url::URL;

pub struct Tab{
    pub displayList: Vec<String>,
    text: String,
    pub(crate) cache: Cache
}

#[derive(Debug, Clone)]
pub enum Message {
    // ScrollDown, 
    // Tab(Event),
    
}



impl Tab {
    pub fn new(url: URL) -> Self {
        let (text, dislist): (String, Vec<String>) = Self::load(url);

        Self{
            displayList: dislist,
            text,
            cache: canvas::Cache::default()
        }
    }

    // pub fn run(url: URL) -> iced::Result {
    //     let boot = move || Browser::new(url.clone());

    //     iced::application(boot, Browser::update, Browser::view).settings(Settings::default()).run()
    // }

    // fn redraw(&mut self) {
    //     self.cache.clear();
    // }

    // TODO - might need to remove this
    fn load(url: URL) -> (String, Vec<String>){
        let content: String = url.request();
        let (text, dis_list): (String, Vec<String>) = lex(content);


        // show(content)
        return (text, dis_list);
    }

    fn layout(mut self) {
    // let disp_list: Vec<(Point, String)> = Vec::new();
        let mut disp_list = Vec::new();
        for mut c in self.text.chars() {
            print!("characters: {:#?}", c)
            // disp_list.append();
        }

        self.displayList = disp_list
    }

}




// impl<Message> canvas::Program<Message> for Browser {
//     type State = ();

//     fn draw(
//         &self,
//         _state: &(),
//         renderer: &Renderer,
//         _theme: &Theme,
//         bounds: Rectangle,
//         _cursor: mouse::Cursor
//     ) -> Vec<canvas::Geometry> {
//         // We prepare a new `Frame`
//         let mut frame = canvas::Frame::new(renderer, bounds.size());

//         println!("Insiode draw maybe?");

//         // We create a `Path` representing a simple circle
//         // let circle = canvas::Path::circle(frame.center(), self.radius);

//         let dpList  = self.displayList.clone();
//         let mut y = 0.0;
//         for text in dpList {
//             frame.fill_text(canvas::Text{
//                 content: text,
//                 position: Point{x:0.0, y:0.0},
//                 max_width: bounds.width,
//                 color: Color::BLACK,
//                 size: Pixels(16.0),
//                 line_height: text::LineHeight::Relative(1.2),
//                 font: Font::DEFAULT,
//                 align_x: text::Alignment::Default,
//                 align_y: iced::alignment::Vertical::Top,
//                 shaping:  text::Shaping::Basic,
//             });
//         }
//         // frame.fill_text(text);


//         // println!("{:#?}", frame);

//         // And fill it with some color
//         // frame.fill(&circle, Color::BLACK);

//         // Then, we produce the geometry
//         vec![frame.into_geometry()]
//     }
// }



fn lex(body: String)  -> (String, Vec<String>) {   
    let mut text: String = "".to_string();
    let mut displ_list: Vec<String> = Vec::new();
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>'{
            in_tag = false;
        } else if !in_tag {
            text.push(c);

            displ_list.push(c.to_string());
            // print!("{}", c);
        }
    }
    (text, displ_list)
}
