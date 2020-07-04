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
    l_in: InputPort<Audio>,
    r_in: InputPort<Audio>,
    l_out: OutputPort<Audio>,
    r_out: OutputPort<Audio>,
    ctl_delay: InputPort<Control>,
    depth: InputPort<Control>,
    rate: InputPort<Control>,
    phase: InputPort<Control>,
    feedback: InputPort<Control>,
}

#[uri("urn:yru-rust-lv2-plugins:yru-flanger-stereo-rs")]
struct YruFlangerRs {
    l_rb: dasp_ring_buffer::Fixed<Vec<f32>>,
    r_rb: dasp_ring_buffer::Fixed<Vec<f32>>,
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
        let l_rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let r_rb = dasp_ring_buffer::Fixed::from(vec![0f32; rb_size]);
        let progression = 0.0;
        Some(Self {
            l_rb,
            r_rb,
            sr,
            progression,
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures) {
        let avg_delay_smpl = *ports.ctl_delay * 1E-3 * self.sr;
        let depth = *ports.depth;
        let rate_smpl = *ports.rate / self.sr;
        let phase_rad = *ports.phase * PI / 180.0;
        let feedback = *ports.feedback;
        for (s_in, s_out) in Iterator::zip(
            Iterator::zip(ports.l_in.iter(), ports.r_in.iter()),
            Iterator::zip(ports.l_out.iter_mut(), ports.r_out.iter_mut()),
        ) {
            //lfo out, control of the delay line. A least one sample of delay is required for
            //feeback
            let omega = 2.0 * PI * self.progression;
            let l_delay_smpl = (f32::sin(omega) * depth + 1.0) * avg_delay_smpl + 1.0;
            let r_delay_smpl = (f32::sin(omega + phase_rad) * depth + 1.0) * avg_delay_smpl + 1.0;
            self.progression += rate_smpl;
            if self.progression > 1.0 {
                self.progression -= 1.0;
            }

            //left
            let l_delay_smpl_i = l_delay_smpl.floor(); // integral part
            let l_delay_smpl_d = l_delay_smpl - l_delay_smpl_i; // decimal part

            let l_rb_index_a =
                self.l_rb.len() - (l_delay_smpl_i as usize).max(1).min(self.l_rb.len());
            let l_rb_index_b =
                self.l_rb.len() - (l_delay_smpl_i as usize + 1).max(1).min(self.l_rb.len());
            let l_delay_out = *self.l_rb.get(l_rb_index_a) * (1.0 - l_delay_smpl_d)
                + *self.l_rb.get(l_rb_index_b) * l_delay_smpl_d;
            let l_out = *s_in.0 + l_delay_out * feedback;

            self.l_rb.push(l_delay_out * feedback + *s_in.0);
            *s_out.0 = l_out; // mix * delay_out + (1.0 - mix) * (*s_in);

            //right
            let r_delay_smpl_i = r_delay_smpl.floor(); // integral part
            let r_delay_smpl_d = r_delay_smpl - r_delay_smpl_i; // decimal part

            let r_rb_index_a =
                self.r_rb.len() - (r_delay_smpl_i as usize).max(1).min(self.r_rb.len());
            let r_rb_index_b =
                self.r_rb.len() - (r_delay_smpl_i as usize + 1).max(1).min(self.r_rb.len());
            let r_delay_out = *self.r_rb.get(r_rb_index_a) * (1.0 - r_delay_smpl_d)
                + *self.r_rb.get(r_rb_index_b) * r_delay_smpl_d;
            let r_out = *s_in.1 + r_delay_out * feedback;

            self.r_rb.push(r_delay_out * feedback + *s_in.0);
            *s_out.1 = r_out; // mix * delay_out + (1.0 - mix) * (*s_in);
        }
    }
}

lv2_descriptors!(YruFlangerRs);
