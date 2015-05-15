//! Provides an acyclic graph for connecting devices.

use std::vec::Vec;

mod types;
use types::{Device};


pub struct DeviceGraph {
    devices: Vec<Box<Device<Input=f32, Output=f32>>>,
}

impl DeviceGraph {
    /// Create an empty DeviceGraph
    pub fn new() -> DeviceGraph {
        DeviceGraph {
            devices: Vec::new(),
        }
    }
}
