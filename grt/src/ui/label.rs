use std::rc::Rc;

use ui::theme::{HorizontalTextAlignment, VerticalTextAlignment};
use ui::{Widget, WidgetKind};
use io::{GraphicsRenderer, TextRenderer};
use util::Point;

pub struct Label {
    pub text: Option<String>,
}

impl Label {
    pub fn empty() -> Rc<Label> {
        Rc::new(Label {
            text: None,
        })
    }

    pub fn new(text: &str) -> Rc<Label> {
        Rc::new(Label {
            text: Some(text.to_string()),
        })
    }

    fn get_draw_params(width: f32, widget: &Widget) -> (f32, f32, &str) {
        let text = &widget.state.text;
        let x = widget.state.inner_left() as f32;
        let y = widget.state.inner_top() as f32;
        let w = widget.state.inner_size.width as f32;
        let h = widget.state.inner_size.height as f32;

        let len = if width > w as f32 {
            w
        } else {
            width
        };

        let x = match widget.state.horizontal_text_alignment {
            HorizontalTextAlignment::Left => x,
            HorizontalTextAlignment::Center => (x + (w - len) / 2.0),
            HorizontalTextAlignment::Right => x - len,
        };

        let y = match widget.state.vertical_text_alignment {
            VerticalTextAlignment::Top => y,
            VerticalTextAlignment::Center => y + (h - 1.0) / 2.0,
            VerticalTextAlignment::Bottom => y + h - 1.0,
        };

        (x, y, &text)
    }
}

impl WidgetKind for Label {
    fn get_name(&self) -> &str {
        "label"
    }

    fn layout(&self, widget: &mut Widget) {
        if let Some(ref text) = self.text {
            widget.state.add_text_param(text);
        }

        widget.do_base_layout();
    }

    fn draw_graphics_mode(&self, renderer: &mut GraphicsRenderer, _pixel_size: Point,
                          widget: &Widget, _millis: u32) {
        let font = match &widget.state.font {
            &None => return,
            &Some(ref font) => font,
        };
        let width = font.get_width(&widget.state.text) as f32 / font.base as f32 * 1.0;
        let (x, y, text) = Label::get_draw_params(width, widget);

        let mut draw_list = font.get_draw_list(text, x, y, 1.0);
        draw_list.set_color(widget.state.text_color);

        renderer.draw(draw_list);
    }

    fn draw_text_mode(&self, renderer: &mut TextRenderer, widget: &Widget,
                      _millis: u32) {
        let (x, y, text) = Label::get_draw_params(widget.state.text.len() as f32, widget);
        renderer.set_cursor_pos(x as i32, y as i32);
        renderer.render_string(&text);
    }
}
