mod library;

use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::EventLoop};

use vulkano::{
    device::{Device, Queue},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
};

fn main() {
    let instance = Instance::new(
        None,
        &InstanceExtensions::supported_by_core().unwrap(),
        None,
    )
    .expect("failed to create vulkan instance");

    let physdevs: Vec<String> = library::get_phys_vec(&instance).unwrap();
}
