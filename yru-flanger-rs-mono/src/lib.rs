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
const CTL_MAX_DELAY_SEC: f32 = 10E-3;

#[derive(PortCollection)]
struct Ports {
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
    ctl_delay: InputPort<Control>,
    depth: InputPort<Control>,
    rate: InputPort<Control>,
    feedback: InputPort<Control>,
}

#[uri("urn:yru-rust-lv2-plugins:yru-flanger-rs-mono")]
struct YruFlangerRs {
    rb: dasp_ring_buffer::Fixed<Vec<f32>>,
    sr: f32,
    progression: f32, // progression of modulation
}

impl Plugin for YruFlangerRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        let sr = plugin_info.sample_rate() as _;
        //trailing +2 : feedback delay + headroom
        let rb_size =
            (plugin_info.sample_rate() as f32 * 2.0 * CTL_MAX_DELAY_SEC).ceil() as usize + 2;
        let rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let progression = 0.0;
        Some(Self {
            rb,
            sr,
            progression,
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let avg_delay_smpl = *ports.ctl_delay * 1E-3 * self.sr;
        let depth = *ports.depth;
        let rate_smpl = *ports.rate / self.sr;
        let feedback = *ports.feedback;
        for (s_in, s_out) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            //lfo out, control of the delay line. A least one sample of delay is required for
            //feeback
            let delay_smpl = (f32::sin(2.0 * PI * self.progression) * depth + 1.0) * avg_delay_smpl + 1.0;
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }

            let delay_smpl_i = delay_smpl.floor(); // integral part
            let delay_smpl_d = delay_smpl - delay_smpl_i; // decimal part

            let rb_index_a = self.rb.len() - (delay_smpl_i as usize).max(1).min(self.rb.len());
            let rb_index_b = self.rb.len() - (delay_smpl_i as usize + 1).max(1).min(self.rb.len());
            let delay_out = *self.rb.get(rb_index_a) * (1.0 - delay_smpl_d)
                + *self.rb.get(rb_index_b) * delay_smpl_d;
            let out = *s_in+ delay_out*feedback;

            self.rb.push(delay_out*feedback + s_in);
            *s_out = out;// mix * delay_out + (1.0 - mix) * (*s_in);
        }
    }
}

lv2_descriptors!(YruFlangerRs);
