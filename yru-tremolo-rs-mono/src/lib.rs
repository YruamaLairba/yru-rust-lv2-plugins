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
use std::f32::consts::PI;

#[derive(PortCollection)]
struct Ports {
    depth: InputPort<Control>,
    rate: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

/// A plugin to demonstrate how to make preset. This is fully handled by rdf spec, so the plugin
/// does nothing.
#[uri("urn:yru-rust-lv2-plugins:yru-tremolo-rs-mono")]
struct YruTremoloRs {
    sr: f32,
    progression: f32, // progression of modulation
}

impl Plugin for YruTremoloRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let sr = plugin_info.sample_rate() as _;
        let progression = 0.0;
        Some(Self { sr, progression })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let depth = *ports.depth;
        let rate_smpl = *ports.rate / self.sr;
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            let gain = 1.0 - depth * 0.5*(f32::sin(2.0 * PI * self.progression) + 1.0);
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }
            *s_out = gain * s_in;
        }
    }
}

lv2_descriptors!(YruTremoloRs);
