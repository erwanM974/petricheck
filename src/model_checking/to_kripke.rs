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

use citreelo::kripke::{KripkeState, KripkeStructure};
use map_macro::hash_map;

use crate::model::marking::Marking;
use crate::model::net::PetriNet;
use crate::model::transition::PetriTransition;
use crate::model_checking::state::PetriKripkeState;




pub trait PetriKripkeStateProducer {
    fn try_reach_new_state(
        &self,
        net_place_num : usize,
        initial : &PetriKripkeState, 
        transition_id : usize,
        transition : &PetriTransition
    ) -> Option<PetriKripkeState>;
}



pub struct DefaultPetriKripkeStateProducer {
    transition_id_to_label_id : HashMap<usize,usize>
}

impl DefaultPetriKripkeStateProducer {
    pub fn new(transition_id_to_label_id: HashMap<usize,usize>) -> Self {
        Self { transition_id_to_label_id }
    }
}


impl PetriKripkeStateProducer for DefaultPetriKripkeStateProducer 
    {
    fn try_reach_new_state(
        &self,
        net_place_num : usize,
        initial : &PetriKripkeState, 
        transition_id : usize,
        transition : &PetriTransition
    ) -> Option<PetriKripkeState> {
        if let Some(new_marking) = transition.try_fire(net_place_num, &initial.marking) {
            let label_id = self.transition_id_to_label_id.get(&transition_id).copied();
            Some(
                PetriKripkeState::new(new_marking, label_id)
            )
        } else {
            None 
        }
    }
}


pub fn petri_to_kripke<
    StateProducer : PetriKripkeStateProducer
>(
    petri : &PetriNet, 
    initial_marking : Marking,
    state_producer : StateProducer
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
    while let Some(origin_state) = queue.pop() {
        let origin_state_id = *states_map.get(&origin_state).unwrap();
        for (transition_id,transition) in petri.transitions.iter().enumerate() {
            if let Some(target_state) = state_producer.try_reach_new_state(
                petri.num_places, 
                &origin_state, 
                transition_id,
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



