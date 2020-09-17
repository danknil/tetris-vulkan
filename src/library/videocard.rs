extern crate vulkano;

use std::{iter::Iterator, sync::Arc};

use vulkano::{
    device::{Device, Queue, DeviceExtensions},
    instance::PhysicalDevice,
};
pub struct Videocard {
    pub physical: PhysicalDevice,
    pub queue: Arc<Queue>,
    pub device: Arc<Device>,
}

impl Videocard {
    pub fn new<'a>(instance: &'a Arc<Instance>, nameOfDev: &'a str) -> Self {
        let physical = PhysicalDevice::enumerate(&instance).find(|x| x.name == nameOfDev).unwrap();
        let queue_family = physical.queues
        let (device, mut queues) = Device::new(&physical, physical.supported_features(),  , )
    }
}
