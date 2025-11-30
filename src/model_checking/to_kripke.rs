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


use std::collections::{HashSet};

use citreelo::kripke::{KripkeState, KripkeStructure};
use map_macro::hash_map;

use crate::model::label::PetriTransitionLabel;
use crate::model::marking::Marking;
use crate::model::net::PetriNet;
use crate::model::transition::PetriTransition;
use crate::model_checking::state::PetriKripkeState;



pub struct PetriKripkeStateProducer {
    tagged_transition_labels : HashSet<PetriTransitionLabel>
}

impl PetriKripkeStateProducer {
    pub fn new(tagged_transition_labels: HashSet<PetriTransitionLabel>) -> Self {
        Self { tagged_transition_labels }
    }


    pub fn try_reach_new_state(
        &self,
        net_place_num : usize,
        initial : &PetriKripkeState,
        transition : &PetriTransition
    ) -> Option<PetriKripkeState> {
        if let Some(new_marking) = transition.try_fire(net_place_num, &initial.marking) {
            let previous_transition_tag_id = match &transition.transition_label {
                None => {None},
                Some(lab_ref) => {
                    if self.tagged_transition_labels.contains(lab_ref) {
                        Some(lab_ref.clone())
                    } else {
                        None
                    }
                }
            };
            Some(
                PetriKripkeState::new(new_marking, previous_transition_tag_id)
            )
        } else {
            None 
        }
    }
}

pub fn petri_to_kripke(
    petri : &PetriNet, 
    initial_marking : Marking,
    state_producer : &PetriKripkeStateProducer
) -> KripkeStructure<PetriKripkeState> {
    let (mut states, mut states_map, mut queue) = {
        let initial_state = PetriKripkeState::new(initial_marking.clone(), None);
        let states_map = hash_map!{
            initial_state.clone() => 0
        };
        let states = vec![
            KripkeState::new(initial_state.clone(),Vec::new())
        ];
        let queue = vec![initial_state];
        (states,states_map,queue)
    };
    let net_num_places = petri.places.len();
    while let Some(origin_state) = queue.pop() {
        let origin_state_id = *states_map.get(&origin_state).unwrap();
        for transition in petri.transitions.iter(){
            if let Some(target_state) = state_producer.try_reach_new_state(
                net_num_places, 
                &origin_state, 
                transition
            ) {
                let target_state_id = match states_map.get(&target_state) {
                    None => {
                        let id = states.len();
                        states.push(KripkeState::new(target_state.clone(), Vec::new()));
                        states_map.insert(target_state.clone(), id);
                        queue.push(target_state);
                        id
                    },
                    Some(id) => {
                        *id
                    }
                };
                let origin_state = states.get_mut(origin_state_id).unwrap();
                if !origin_state.outgoing_transitions_targets.contains(&target_state_id) {
                    origin_state.outgoing_transitions_targets.push(target_state_id);
                }
            }
        }
    }
    KripkeStructure::new(states)
}



