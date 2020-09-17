extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use std::sync::Arc;
use vulkano::instance::Instance;
use winit::{
    event_loop::EventLoop,
    window::{Icon, WindowBuilder},
};

pub struct CreatedWindow {
    width: u32,
    height: u32,
    title: String,
    icon: winit::window::Icon,
}

impl CreatedWindow {
    pub fn new<T: Into<String>>(
        instance: &Arc<Instance>,
        width: u32,
        height: u32,
        title: T,
        //event_loop: &EventLoop,
        icon: Option<&Icon>,
    ) -> Option<Self> {
        let builder = WindowBuilder::new();
        None
    }
}
