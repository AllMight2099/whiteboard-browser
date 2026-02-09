use crate::tab::{Message, Tab};

use iced::advanced::graphics::geometry::Renderer;
use iced::advanced::widget::Tree;
use iced::widget::canvas::Text;
use iced::widget::container;
use iced::advanced::{Widget, renderer};
use iced::{Element, Point, Size, Theme};
use iced::Length;
use iced::mouse::Cursor;

// use iced_core::layout::Layout;
use iced_core::layout;

const HSTEP: f32 = 13.0;
const VSTEP: f32 = 18.0;

pub struct TabView<'a> {
    tab: &'a Tab,
}

impl<'a> TabView<'a> {
    pub fn show(tab: &'a Tab) -> Element<'a, Message> {
        println!("inside show");
        container(Self{tab}).into()
    }
}

impl Widget<Message,Theme, iced::Renderer> for TabView <'_> {
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: Length::Fill, 
            height: Length::Fill,
        }
    }

    // need to confirm what this does
    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &iced_core::layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        iced::advanced::layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: Cursor,
        viewport: &iced::Rectangle,
    ) {
        let geometry = self.tab.cache.draw(renderer, viewport.size(), |frame| {
        
            let mut position: Point = Point { x: 0.0, y: 0.0 };
            for content in self.tab.displayList.iter() {
                // println!("character inside content: {:#?}", c);
                let text = Text{
                    content: content.to_string(),
                    position,
                    size: iced::Pixels(20.0),
                    color: iced::Color::WHITE, 
                    // line_height: todo!(),
                    align_x: iced_core::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    // shaping: iced_core::text::Shaping::Advanced,
                    ..Default::default()
                };

                frame.fill_text(text);

                position.x += HSTEP;
                
                if position.x > viewport.width {
                    position.y += VSTEP;
                    position.x = 0.0;
                }
            }

        });

        

        renderer.draw_geometry(geometry);
    }

}


impl<'a> From<TabView<'a>> for Element<'a, Message, Theme, iced::Renderer> {
    fn from(widget: TabView<'a>) -> Self {
        Self::new(widget)
    }
}