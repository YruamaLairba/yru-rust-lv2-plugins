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
    l_in: InputPort<Audio>,
    r_in: InputPort<Audio>,
    l_out: OutputPort<Audio>,
    r_out: OutputPort<Audio>,
    ctl_delay: InputPort<Control>,
    depth: InputPort<Control>,
    rate: InputPort<Control>,
    phase: InputPort<Control>,
    mix: InputPort<Control>,
}

#[uri("urn:yru-rust-lv2-plugins:yru-chorus-stereo-rs")]
struct YruChorusRs {
    l_rb: dasp_ring_buffer::Fixed<Vec<f32>>, //delay line for left channel
    r_rb: dasp_ring_buffer::Fixed<Vec<f32>>, //delay line for right channel
    rb_max_i: usize,                         //last rb index
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
        let l_rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let r_rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let progression = 0.0;
        Some(Self {
            l_rb,
            r_rb,
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
        let phase_rad = *ports.phase * PI / 180.0;
        for (s_in, s_out) in Iterator::zip(
            Iterator::zip(ports.l_in.iter(), ports.r_in.iter()),
            Iterator::zip(ports.l_out.iter_mut(), ports.r_out.iter_mut()),
        ) {
            self.l_rb.push(*s_in.0);
            self.r_rb.push(*s_in.1);
            //lfo out, control of the delay line
            let omega = 2.0 * PI * self.progression;
            let l_delay_smpl = (f32::sin(omega) * depth + 1.0) * avg_delay_smpl;
            let r_delay_smpl = (f32::sin(omega + phase_rad) * depth + 1.0) * avg_delay_smpl;
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }

            let l_delay_smpl_i = l_delay_smpl.floor(); // integral part
            let r_delay_smpl_i = r_delay_smpl.floor(); // integral part
            let l_delay_smpl_d = l_delay_smpl - l_delay_smpl_i; // decimal part
            let r_delay_smpl_d = r_delay_smpl - r_delay_smpl_i; // decimal part

            let l_rb_index_a = self.rb_max_i - (l_delay_smpl_i as usize).min(self.rb_max_i);
            let r_rb_index_a = self.rb_max_i - (r_delay_smpl_i as usize).min(self.rb_max_i);
            let l_rb_index_b = self.rb_max_i - (l_delay_smpl_i as usize + 1).min(self.rb_max_i);
            let r_rb_index_b = self.rb_max_i - (r_delay_smpl_i as usize + 1).min(self.rb_max_i);
            let l_delay_out = *self.l_rb.get(l_rb_index_a) * (1.0 - l_delay_smpl_d)
                + *self.l_rb.get(l_rb_index_b) * l_delay_smpl_d;
            let r_delay_out = *self.r_rb.get(r_rb_index_a) * (1.0 - r_delay_smpl_d)
                + *self.r_rb.get(r_rb_index_b) * r_delay_smpl_d;

            *s_out.0 = mix * l_delay_out + (1.0 - mix) * (*s_in.0);
            *s_out.1 = mix * r_delay_out + (1.0 - mix) * (*s_in.1);
        }
    }
}

lv2_descriptors!(YruChorusRs);
