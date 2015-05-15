pub type Sample = f32;
pub trait Device {
    type Input;
    type Output;

    /// Return the number of inputs this device will accept
    fn num_inputs(&self) -> usize;

    /// Return the number of outputs this device generates
    fn num_outputs(&self) -> usize;
}
