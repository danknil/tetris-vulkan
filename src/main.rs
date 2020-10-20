mod Models;
mod Render;

use crate::Models::{figure::AbstractFigure, map::TetrisMap};
use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::EventLoop};

use vulkano::{
    device::{Device, Features, Queue, RawDeviceExtensions},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
};

fn main() {
    let instance = Instance::new(
        None,
        &InstanceExtensions::supported_by_core().unwrap(),
        None,
    )
    .expect("failed to create vulkan instance");

    let physdev = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("physical device doesn't exist");

    println!(
        "Using device named: {}\nType: {:?}",
        physdev.name(),
        physdev.ty(),
    );

    let queue_families = [physdev
        .queue_families()
        .find(|q| q.supports_graphics())
        .unwrap()];

    let (device, mut queue) = Device::new(
        physdev,
        &Features::default(),
        RawDeviceExtensions::supported_by_device(physdev),
        queue_families.iter().cloned().zip([0.5].iter().cloned()),
    )
    .expect("can't make device");
}

fn init_map() -> TetrisMap {
    let figures: Vec<AbstractFigure> = vec![];
    TetrisMap::new(figures, 10, 10, 20)
}
