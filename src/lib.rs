//! Provides an acyclic graph for connecting devices.

use std::default::Default;
use std::marker::PhantomData;
use std::vec::Vec;

mod types;
use types::{Device, Sample};


/// Tracks the state and connections of many devices as an acyclic graph.
pub struct DeviceGraph {
    devices: Vec<NodeWrapper>,
}

impl DeviceGraph {
    /// Create an empty DeviceGraph
    pub fn new() -> DeviceGraph {
        DeviceGraph {
            devices: Vec::new(),
        }
    }

    /// Link two devices
    pub fn add_edge<I, E, O>(&mut self,
                             from: DeviceNode<I, E>, from_ch: usize,
                             to: DeviceNode<E, O>, to_ch: usize)
            -> Result<(), &'static str> {
        let DeviceNode(i, _, _) = from;
        let DeviceNode(j, _, _) = to;
        let ch = try!(self.devices[i].get_output_channel(from_ch));
        self.devices[j].set_input_channel(to_ch, ch)
    }
}

pub struct DeviceNode<I, O> (usize, PhantomData<I>, PhantomData<O>);

pub trait AddDevice<I, O> {
    fn add<T: 'static+Device<Input=I, Output=O>>(&mut self, d: T)
        -> DeviceNode<I, O>;
}

macro_rules! make_device_types {
    (
        $( $name:ident: $input:ty => $output:ty),+
    ) => {
        enum NodeWrapper {
            $( $name(Node<$input, $output>) ),+
        }

        impl NodeWrapper {
            fn get_output_channel(&self, ch: usize)
                    -> Result<usize, &'static str> {
                match self {
                    $( &NodeWrapper::$name(ref n) => {
                        if ch < n.num_outputs {
                            Ok(n.outputs[ch])
                        } else {
                            Err("not enough channels")
                        }
                    }),+
                }
            }

            fn set_input_channel(&mut self, ch: usize, input: usize)
                    -> Result<(), &'static str> {
                match self {
                    $( &mut NodeWrapper::$name(ref mut n) => {
                        if ch < n.device.num_inputs() {
                            n.inputs.push(input);
                            Ok(())
                        } else {
                            Err("not enough channels")
                        }
                    }),+
                }
            }
        }

        $(
            impl AddDevice<$input, $output> for DeviceGraph {
                fn add<T: 'static+Device<Input=$input, Output=$output>>
                        (&mut self, d: T) -> DeviceNode<$input, $output> {
                    DeviceNode(self.devices.len(), PhantomData, PhantomData)
                }
            }
        )+
    }
}

make_device_types!(
    SampleToSample: Sample => Sample
);


struct Node<I, O> {
    device: Box<Device<Input=I, Output=O>>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    num_outputs: usize
}

impl<I, O> Node<I, O> {
    fn from_device<T: 'static+Device<Input=I, Output=O>>(d: T) -> Node<I, O> {
        let num_inputs = d.num_inputs();
        let num_outputs = d.num_inputs();
        Node {
            device: Box::new(d),
            inputs: Vec::with_capacity(num_inputs),
            outputs: Vec::with_capacity(num_outputs),
            num_outputs: num_outputs
        }
    }
}
