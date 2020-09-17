mod wincreate;

use std::sync::Arc;
use vulkano::instance::{Instance, PhysicalDevice};

pub fn get_phys_vec(instance: &Arc<Instance>) -> Option<Vec<String>> {
    let mut x: Vec<String> = Vec::new();

    match PhysicalDevice::enumerate(&instance).next() {
        None => return None,
        _ => { for y in PhysicalDevice::enumerate(instance) { x.push(y.name().to_string()) } } // push names to vector
    };
    Some(x) // return a vector
}
