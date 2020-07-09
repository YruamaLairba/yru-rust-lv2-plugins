/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use lv2_core::prelude::*;
use urid::*;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
    delay: InputPort<Control>,
    feedback: InputPort<Control>,
    mix: InputPort<Control>,
}

/// A plugin to demonstrate how to make preset. This is fully handled by rdf spec, so the plugin
/// does nothing.
#[uri("urn:yru-rust-lv2-plugins:yru-echo-rs-mono")]
struct YruEchoRs {
    sr: f32,
    rb: dasp_ring_buffer::Fixed<Vec<f32>>,
}

impl Plugin for YruEchoRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let sr = plugin_info.sample_rate() as _;
        let max_delay_s = (plugin_info.sample_rate() * 2.0).ceil() as _;
        let rb = dasp_ring_buffer::Fixed::from(vec![ 0f32; max_delay_s ]);
        Some(Self { sr, rb })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let delay_s = self.sr * (*ports.delay as f32) * 0.001;
        let feedback = *ports.feedback;
        let mix = *ports.mix;
        let rb_index = self.rb.len() - (delay_s as usize).max(1).min(self.rb.len());
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            let delay_out = *self.rb.get(rb_index);
            self.rb.push(*s_in+feedback*delay_out);
            *s_out = mix*delay_out + (1.0-mix)*(*s_in);
        }
    }
}

lv2_descriptors!(YruEchoRs);
