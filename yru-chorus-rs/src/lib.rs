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
use std::f32::consts::PI;
use urid::*;

// maximum variation in delay line
const MAX_CHORUS_AMPLITUDE_SEC: f32 = 0.030;

const MIN_DELAY_SEC: f32 = 0.010;
const MAX_DELAY_SEC: f32 = MIN_DELAY_SEC + MAX_CHORUS_AMPLITUDE_SEC;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
    rate: InputPort<Control>,
    depth: InputPort<Control>,
    mix: InputPort<Control>,
}

#[uri("urn:yru-rust-lv2-plugins:yru-chorus-rs")]
struct YruChorusRs {
    rb: dasp_ring_buffer::Fixed<Vec<f32>>,
    sr: f32,
    progression: f32, // progression of modulation
}

impl Plugin for YruChorusRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let sr = plugin_info.sample_rate() as _;
        let max_delay_smpl = (plugin_info.sample_rate() as f32 * MAX_DELAY_SEC).ceil() as _;
        let rb = dasp_ring_buffer::Fixed::from(vec![0f32; max_delay_smpl]);
        let progression = 0.0;
        Some(Self {
            rb,
            sr,
            progression,
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let rate_smpl = *ports.rate / self.sr;
        let depth = *ports.depth;
        let mix = *ports.mix;
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            //lfo out, control of the delay line
            let delay_smpl = (0.5
                * (1.0 + f32::sin(2.0 * PI * self.progression))
                * depth
                * MAX_CHORUS_AMPLITUDE_SEC
                + MIN_DELAY_SEC)
                * self.sr;
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }

            let rb_index = self.rb.len() - (delay_smpl as usize).max(1).min(self.rb.len());
            let delay_out = *self.rb.get(rb_index);
            self.rb.push(*s_in);
            *s_out = mix * delay_out + (1.0 - mix) * (*s_in);
        }
    }
}

lv2_descriptors!(YruChorusRs);
