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

// maximum delay on the delay knob
const CTL_MAX_DELAY_SEC: f32 = 20E-3;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
    ctl_delay: InputPort<Control>,
    depth: InputPort<Control>,
    rate: InputPort<Control>,
    mix: InputPort<Control>,
}

#[uri("urn:yru-rust-lv2-plugins:yru-chorus-rs")]
struct YruChorusRs {
    rb: dasp_ring_buffer::Fixed<Vec<f32>>,
    rb_max_i: usize, //last rb index
    sr: f32,
    progression: f32, // progression of modulation
}

impl Plugin for YruChorusRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let sr = plugin_info.sample_rate() as _;
        //the trailing +1 is a headroom to absorb rounding error during delay calculation 
        let rb_max_i =
            (plugin_info.sample_rate() as f32 * 2.0 * CTL_MAX_DELAY_SEC).ceil() as usize + 1;
        let rb_size = rb_max_i + 1;
        let rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let progression = 0.0;
        Some(Self {
            rb,
            rb_max_i,
            sr,
            progression,
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let avg_delay_smpl = *ports.ctl_delay * 1E-3 * self.sr;
        let depth = *ports.depth;
        let rate_smpl = *ports.rate / self.sr;
        let mix = *ports.mix;
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            self.rb.push(*s_in);
            //lfo out, control of the delay line
            let delay_smpl = (f32::sin(2.0 * PI * self.progression) * depth + 1.0) * avg_delay_smpl;
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }

            let delay_smpl_i = delay_smpl.floor(); // integral part
            let delay_smpl_d = delay_smpl - delay_smpl_i; // decimal part

            let rb_index_a = self.rb_max_i - (delay_smpl_i as usize).min(self.rb_max_i);
            let rb_index_b = self.rb_max_i - (delay_smpl_i as usize + 1).min(self.rb_max_i);
            let delay_out = *self.rb.get(rb_index_a) * (1.0 - delay_smpl_d)
                + *self.rb.get(rb_index_b) * delay_smpl_d;

            *s_out = mix * delay_out + (1.0 - mix) * (*s_in);
        }
    }
}

lv2_descriptors!(YruChorusRs);
