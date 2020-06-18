use lv2_core::prelude::*;
use urid::*;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<Audio>,
    output:OutputPort<Audio>,
    _delay: InputPort<Control>,
    _feedback: InputPort<Control>,
    _mix: InputPort<Control>,
}

/// A plugin to demonstrate how to make preset. This is fully handled by rdf spec, so the plugin
/// does nothing.
#[uri("urn:yru-rust-lv2-plugins:yru-echo-rs")]
struct YruEchoRs {
    rb: dasp_ring_buffer::Fixed<Vec<f32>>,
}

impl Plugin for YruEchoRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let rb = dasp_ring_buffer::Fixed::from(vec![0f32;(plugin_info.sample_rate()*5.0).ceil() as usize+1]);
        Some(Self {rb})
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *s_out = *self.rb.get(0);
            self.rb.push(*s_in);
        }

    }
}

lv2_descriptors!(YruEchoRs);
