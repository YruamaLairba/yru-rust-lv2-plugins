use lv2_core::prelude::*;
use urid::*;

#[derive(PortCollection)]
struct Ports {
    _input: InputPort<Audio>,
    _output:OutputPort<Audio>,
    _delay: InputPort<Control>,
    _feedback: InputPort<Control>,
    _mix: InputPort<Control>,
}

/// A plugin to demonstrate how to make preset. This is fully handled by rdf spec, so the plugin
/// does nothing.
#[uri("urn:yru-rust-lv2-plugins:yru-echo-rs")]
struct YruEchoRs {}

impl Plugin for YruEchoRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        Some(Self {})
    }

    fn run(&mut self, _ports: &mut Ports, _features: &mut Self::AudioFeatures) {}
}

lv2_descriptors!(YruEchoRs);
