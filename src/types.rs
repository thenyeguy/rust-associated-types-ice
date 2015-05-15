//! Defines types and constants to be used globally in oxcable.

use std::vec::Vec;

/// The datatype of a single sample time.
pub type Time = u64;

/// The datatype of a single sample.
pub type Sample = f32;


/// Defines the ways a Device can return and receive data to process.
#[derive(Clone, Copy, Debug)]
pub enum DeviceIOType {
    Any,
    Exactly(usize),
}

/// An interface for a synchronous processing device.
pub trait Device {
    type Input;
    type Output;

    /// Return the number of inputs this device will accept
    fn num_inputs(&self) -> DeviceIOType;

    /// Return the number of outputs this device generates
    fn num_outputs(&self) -> DeviceIOType;

    /// Process a single frame worth of data. This function should be called
    /// once per time step, starting at `t=0`.
    fn tick(&mut self, t: Time, inputs: &Vec<Self::Input>,
            outputs: &mut Vec<Self::Output>);
}
