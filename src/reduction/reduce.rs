/*
Copyright 2025 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::model::{marking::Marking, net::PetriNet};
use crate::reduction::info::PetriNetInfo;
use crate::reduction::fusion_series_places::find_and_simplify_series_places;
use crate::reduction::fusion_series_transitions_variant1::find_and_simplify_series_transitions_variant1;
use crate::reduction::fusion_series_transitions_variant2::find_and_simplify_series_transitions_variant2;





pub fn reduce_petri_net(
    petri_net : &mut PetriNet,
    initial_markings : &mut Option<Marking>
) {

    let mut petri_info = PetriNetInfo::from_petri_net(petri_net);

    loop {
        if find_and_simplify_series_transitions_variant1(
            petri_net,
            &mut petri_info,
            initial_markings
        ) {
            #[cfg(debug_assertions)] println!("simplify_series_transitions_variant1");
            continue;
        }
        if find_and_simplify_series_transitions_variant2(
            petri_net,
            &mut petri_info,
            initial_markings
        ) {
            #[cfg(debug_assertions)] println!("simplify_series_transitions_variant2");
            continue;
        }
        if find_and_simplify_series_places(
            petri_net,
            &mut petri_info,
            initial_markings
        ) {
            #[cfg(debug_assertions)] println!("simplify_series_places");
            continue;
        }
        break;
    }
}