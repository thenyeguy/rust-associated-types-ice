//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate minimalice;

#[cfg(not(test))]
fn main() {
    use minimalice::DeviceGraph;
    let graph = DeviceGraph::new();
}
