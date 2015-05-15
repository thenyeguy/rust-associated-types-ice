//! Provides an acyclic graph for connecting devices.

use std::default::Default;
use std::marker::PhantomData;
use std::vec::Vec;

mod types;
use types::{Device, DeviceIOType, Sample, Time};


/// Tracks the state and connections of many devices as an acyclic graph.
pub struct DeviceGraph {
    t: Time,
    devices: Vec<NodeWrapper>,
    bus: GraphBus
}

impl DeviceGraph {
    /// Create an empty DeviceGraph
    pub fn new() -> DeviceGraph {
        DeviceGraph {
            t: 0,
            devices: Vec::new(),
            bus: GraphBus {
                audio_bus: Vec::new(),
                unit_bus: Vec::new(),
            }
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

    /// Process a single time step.
    pub fn tick(&mut self) {
        tick_devices(self);
    }
}

pub struct DeviceNode<I, O> (usize, PhantomData<I>, PhantomData<O>);

pub trait AddDevice<I, O> {
    fn add<T: 'static+Device<Input=I, Output=O>>(&mut self, d: T)
        -> DeviceNode<I, O>;
}

struct GraphBus {
    audio_bus: Vec<Sample>,
    unit_bus: Vec<()>
}

trait GetBus<'a, T> {
    fn get_bus(&'a mut self) -> &'a mut Vec<T>;
}

impl<'a> GetBus<'a, Sample> for GraphBus {
    fn get_bus(&'a mut self) -> &'a mut Vec<Sample> { &mut self.audio_bus }
}

impl<'a> GetBus<'a, ()> for GraphBus {
    fn get_bus(&'a mut self) -> &'a mut Vec<()> { &mut self.unit_bus }
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
                        match n.device.num_inputs() {
                            DeviceIOType::Any => {
                                n.inputs.push(input);
                                Ok(())
                            },
                            DeviceIOType::Exactly(i) => if ch < i {
                                n.inputs.push(input);
                                Ok(())
                            } else {
                                Err("not enough channels")
                            }
                        }
                    }),+
                }
            }
        }

        $(
            impl AddDevice<$input, $output> for DeviceGraph {
                fn add<T: 'static+Device<Input=$input, Output=$output>>
                        (&mut self, d: T) -> DeviceNode<$input, $output> {
                    let mut n = Node::from_device(d);
                    let bus: &mut Vec<$output> = self.bus.get_bus();
                    for i in 0..n.num_outputs {
                        bus.push(Default::default());
                        n.outputs[i] = bus.len();
                    }
                    self.devices.push(NodeWrapper::$name(n));
                    DeviceNode(self.devices.len(), PhantomData, PhantomData)
                }
            }
        )+

        fn tick_devices(graph: &mut DeviceGraph) {
            for device in graph.devices.iter_mut() {
                match device {
                    $( &mut NodeWrapper::$name(ref mut n) => {
                        {
                            let in_bus: &mut Vec<$input> = graph.bus.get_bus();
                            n.input_buffer.clear();
                            for &i in n.inputs.iter() {
                                n.input_buffer.push(in_bus[i].clone());
                            }
                        }
                        n.device.tick(graph.t, &n.input_buffer, 
                                      &mut n.output_buffer);
                        {
                            let out_bus: &mut Vec<$output> = graph.bus.get_bus();
                            for i in 0..n.num_outputs {
                                out_bus[n.outputs[i]] = n.output_buffer[i].clone();
                            }
                        }
                    }),+
                }
            }
        }
    }
}

make_device_types!(
    UnitToSample: () => Sample,
    SampleToSample: Sample => Sample,
    SampleToUnit: Sample => ()
);


struct Node<I, O> {
    device: Box<Device<Input=I, Output=O>>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    input_buffer: Vec<I>,
    output_buffer: Vec<O>,
    num_outputs: usize
}

impl<I, O> Node<I, O> {
    fn from_device<T: 'static+Device<Input=I, Output=O>>(d: T) -> Node<I, O> {
        let num_inputs = match d.num_inputs() {
            DeviceIOType::Any => 0,
            DeviceIOType::Exactly(i) => i,
        };
        let num_outputs = match d.num_outputs() {
            DeviceIOType::Any => panic!("DeviceGraph does not support Any outputs"),
            DeviceIOType::Exactly(i) => i,
        };
        Node {
            device: Box::new(d),
            inputs: Vec::with_capacity(num_inputs),
            outputs: Vec::with_capacity(num_outputs),
            input_buffer: Vec::with_capacity(num_inputs),
            output_buffer: Vec::with_capacity(num_outputs),
            num_outputs: num_outputs
        }
    }
}
