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

use itertools::Itertools;

use crate::{model::{marking::Marking, net::PetriNet}, reduction::info::PetriNetInfo};



pub struct ParallelTransitionsPair {
    pub tx1_id : usize,
    pub tx2_id : usize
}

impl ParallelTransitionsPair {
    pub fn new(tx1_id: usize, tx2_id: usize) -> Self {
        Self { tx1_id, tx2_id }
    }
}




pub fn find_and_simplify_parallel_transitions(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    if let Some(parallel_transitions) = find_parallel_transitions(petri_net) {
        // we simply delete any of the two transitions (let's delete the second)
        
        petri_net.transitions.remove(parallel_transitions.tx2_id);
        petri_info.remove_transition(parallel_transitions.tx2_id);
        true 
    } else {
        false
    }
}

/// parallel transitions are pairs of transitions (t1,t2) such that:
/// - both t1 and t2 have the same label
/// - both t1 and t2 have the same preset map
/// - both t1 and t2 have the same postset map
fn find_parallel_transitions(
    petri_net : &PetriNet
) -> Option<ParallelTransitionsPair> {
    'iter_pairs_of_transitions : for tx_pair in petri_net.transitions.iter().enumerate().combinations(2) {
        let (tx_id1,tx1) = tx_pair.first().unwrap();
        let (tx_id2, tx2) = tx_pair.get(1).unwrap();
        if tx1.transition_label != tx2.transition_label {
            continue 'iter_pairs_of_transitions;
        }
        if tx1.preset_tokens != tx2.preset_tokens {
            continue 'iter_pairs_of_transitions;
        }
        if tx1.postset_tokens != tx2.postset_tokens {
            continue 'iter_pairs_of_transitions;
        }
        return Some(
            ParallelTransitionsPair::new(
                *tx_id1,
                *tx_id2,
            )
        );
    }
    None 
}

