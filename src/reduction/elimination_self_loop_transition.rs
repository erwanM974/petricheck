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


use crate::{model::{marking::Marking, net::PetriNet}, reduction::info::PetriNetInfo};



pub struct SelfLoopTransition {
    pub transition_id : usize
}

impl SelfLoopTransition {
    pub fn new(transition_id: usize) -> Self {
        Self { transition_id }
    }
}


pub fn find_and_simplify_self_loop_transition(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    if let Some(self_loop_transition) = find_self_loop_transition(petri_net) {
        // we remove the self_loop_transition
        petri_net.transitions.remove(self_loop_transition.transition_id);
        petri_info.remove_transition(self_loop_transition.transition_id);
        true
    } else {
        false
    }
}

/// a self-loop transition is a transition t such that:
/// - it has the empty label
/// - its preset and its postset is the same (same places, same numbers of tokens)
///   let's remark that it also eliminate dead transitions (i.e. both preset and postset empty)
fn find_self_loop_transition(
    petri_net : &PetriNet
) -> Option<SelfLoopTransition> {
    for (transition_id, transition) in petri_net.transitions.iter().enumerate() {
        if transition.transition_label.is_none() && transition.postset_tokens == transition.preset_tokens {
            return Some(SelfLoopTransition::new(transition_id));
        }
    }
    None 
}

