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
struct Ports {}

#[uri("urn:yru-rust-lv2-plugins:yru-chorus-rs")]
struct YruChorusRs {}

impl Plugin for YruChorusRs {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
        Some(Self{})
    }

    fn run(&mut self, _ports: &mut Ports, _features: &mut Self::AudioFeatures) {}
}

lv2_descriptors!(YruChorusRs);
