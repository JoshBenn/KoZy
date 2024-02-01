use crate::application::color;


pub enum Component {
    Button(Button),
    Label(Label),
    TextBox(TextBox),
    Image(Image),
    ToolTip(ToolTip),
    Toggle(Toggle),
    Slider(Slider),
    DropdownList(DropdownList),
    ColorPicker(ColorPicker),
    DatePicker(DatePicker),
    ValueInput(ValueInput),
    Table(Table),
    Tree(Tree),
    List(List),
    Modal(Modal),
    Menu(Menu),
    DragAndDrop(DragAndDrop),
    ToolBar(ToolBar),
    MenuBar(MenuBar),
    ProgressBar(ProgressBar),
    Spinner(Spinner),
    Canvas(Canvas),
    StatusBar(StatusBar),
    Accordion(Accordion),
    Space(Space),
    Command(Command),
    Overlay(Overlay),

    Custom(Custom),
}

impl Component {
    // pub fn render(&self) {
    //     match *self {
    //         Component::Button(c) => c.render(),
    //         Component::Label(c) => c.render(),
    //         Component::TextBox(c) => c.render(),
    //         Component::Image(c) => c.render(),
    //         Component::ToolTip(c) => c.render(),
    //         Component::Toggle(c) => c.render(),
    //         Component::Slider(c) => c.render(),
    //         Component::DropdownList(c) => c.render(),
    //         Component::ColorPicker(c) => c.render(),
    //         Component::DatePicker(c) => c.render(),
    //         Component::ValueInput(c) => c.render(),
    //         Component::Table(c) => c.render(),
    //         Component::Tree(c) => c.render(),
    //         Component::List(c) => c.render(),
    //         Component::Modal(c) => c.render(),
    //         Component::Menu(c) => c.render(),
    //         Component::DragAndDrop(c) => c.render(),
    //         Component::ToolBar(c) => c.render(),
    //         Component::MenuBar(c) => c.render(),
    //         Component::ProgressBar(c) => c.render(),
    //         Component::Spinner(c) => c.render(),
    //         Component::Canvas(c) => c.render(),
    //         Component::StatusBar(c) => c.render(),
    //         Component::Accordion(c) => c.render(),
    //         Component::Space(c) => c.render(),
    //         Component::Command(c) => c.render(),
    //         Component::Overlay(c) => c.render(),
    //         Component::Custom(c) => c.render(),
    //     }
    // }
}

#[derive(Clone)]
pub struct Label {

}

pub struct TextBox {

}

pub struct Image {

}

pub struct ToolTip {

}

pub struct Toggle {

}

pub struct Slider {

}

pub struct DropdownList {

}

pub struct ColorPicker {

}

pub struct DatePicker {

}

pub struct ValueInput {

}

pub struct Table {

}

pub struct Tree {

}

pub struct List {

}

pub struct Modal {

}

pub struct Menu {

}

pub struct DragAndDrop {

}

pub struct ToolBar {

}

pub struct MenuBar {

}

pub struct ProgressBar {

}

pub struct Spinner {

}

pub struct Canvas {

}

pub struct Custom {

}

pub struct StatusBar {

}

pub struct Accordion {

}

pub struct Space {

}

pub struct Command {

}

pub struct Overlay {

}

pub struct Button {
    pub label: String,
    pub color: color::Color,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub hover: bool,
}

impl Button {
    pub fn new(label: &str, color: color::Color, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { 
            label: label.to_owned(), 
            color, 
            x, 
            y, 
            width, 
            height, 
            hover: false 
        }
    }

    pub fn inside(&mut self, x: f64, y: f64) -> bool {
        if x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height {
            self.hover = true;
            return true;
        }
        self.hover = false;
        return false;
    }

    pub fn draw(&self) {
        todo!()
    }
}