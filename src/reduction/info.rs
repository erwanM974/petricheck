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


use std::collections::HashMap;

use crate::model::net::PetriNet;



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PetriPlaceInfo {
    pub incoming_transitions : HashMap<usize,u32>,
    pub outgoing_transitions : HashMap<usize,u32>,
}

impl PetriPlaceInfo {
    pub fn new(incoming_transitions: HashMap<usize,u32>, outgoing_transitions: HashMap<usize,u32>) -> Self {
        Self { incoming_transitions, outgoing_transitions }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PetriNetInfo {
    pub places_info  : Vec<PetriPlaceInfo>
}

impl PetriNetInfo {
    pub fn from_petri_net(net : &PetriNet) -> Self {
        let mut places_info : Vec<PetriPlaceInfo> = Vec::new();
        for _ in &net.places {
            places_info.push(PetriPlaceInfo::new(HashMap::new(),HashMap::new()));
        }
        for (tr_id,transition) in net.transitions.iter().enumerate() {
            for (preset_place_id,num_toks) in transition.iter_preset_tokens() {
                let preset_place_info = places_info.get_mut(*preset_place_id).unwrap();
                preset_place_info.outgoing_transitions.insert(tr_id, *num_toks);
            }
            for (postset_place_id,num_toks) in transition.iter_postset_tokens() {
                let postset_place_info = places_info.get_mut(*postset_place_id).unwrap();
                postset_place_info.incoming_transitions.insert(tr_id, *num_toks);
            }
        }
        Self { places_info }
    }

    pub fn remove_transition(&mut self, tr_to_remove_id : usize) {
        for place_info in self.places_info.iter_mut() {
            let mut new_incoming_transitions = HashMap::new();
            for (incoming_tr_id, num_toks) in place_info.incoming_transitions.drain() {
                match usize::cmp(&incoming_tr_id,&tr_to_remove_id) {
                    std::cmp::Ordering::Less => {
                        new_incoming_transitions.insert(incoming_tr_id,num_toks);
                    },
                    std::cmp::Ordering::Equal => {
                        // we remove the transition
                    },
                    std::cmp::Ordering::Greater => {
                        new_incoming_transitions.insert(incoming_tr_id - 1,num_toks);
                    }
                }
            }
            place_info.incoming_transitions = new_incoming_transitions;
            // ***
            let mut new_outgoing_transitions = HashMap::new();
            for (outgoing_tr_id, num_toks) in place_info.outgoing_transitions.drain() {
                match usize::cmp(&outgoing_tr_id,&tr_to_remove_id) {
                    std::cmp::Ordering::Less => {
                        new_outgoing_transitions.insert(outgoing_tr_id,num_toks);
                    },
                    std::cmp::Ordering::Equal => {
                        // we remove the transition
                    },
                    std::cmp::Ordering::Greater => {
                        new_outgoing_transitions.insert(outgoing_tr_id - 1,num_toks);
                    }
                }
            }
            place_info.outgoing_transitions = new_outgoing_transitions;
        }
    }

}




