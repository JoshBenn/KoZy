use crate::components;




pub struct Window {
    pub active: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub components: Vec<components::Component>,
    pub containers: Vec<Container>,
}

pub enum ContainerType {
    Scrollable,
    Column,
    Row,
    Grid,
    Tab,
}

pub struct Container {
    pub layout: ContainerType,
    pub components: Vec<components::Component>,
    pub containers: Vec<Container>
}
