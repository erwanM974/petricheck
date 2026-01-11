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

use std::collections::BTreeMap;
use itertools::Itertools;

use crate::{model::{marking::Marking, net::PetriNet}, reduction::info::PetriNetInfo};



pub struct ParallelPlacesPair {
    #[allow(unused)] 
    pub place1_id : usize,
    pub place2_id : usize,
}

impl ParallelPlacesPair {
    pub fn new(place1_id: usize, place2_id: usize) -> Self {
        Self { place1_id, place2_id }
    }
}



pub fn find_and_simplify_parallel_places(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    if let Some(parallel_places) = find_parallel_places(petri_net, petri_info, initial_markings) {
        // we simply delete any of the two places (let's delete the second)
        if let Some(mark) = initial_markings {
            // we remove the place, shifting the indices
            let mut new_tokens = BTreeMap::new();
            for (place_id,num_toks) in mark.tokens.iter() {
                match usize::cmp(place_id,&parallel_places.place2_id) {
                    std::cmp::Ordering::Less => {
                        new_tokens.insert(*place_id,*num_toks);
                    },
                    std::cmp::Ordering::Equal => {
                        // we remove the place
                    },
                    std::cmp::Ordering::Greater => {
                        new_tokens.insert(*place_id - 1,*num_toks);
                    }
                }
            }
            mark.tokens = new_tokens;
        }

        {
            let place_to_remove_info = petri_info.places_info.get(parallel_places.place2_id).unwrap();
            // we remove the place from the postset of all incoming transitions
            for (incoming_tx_id,_) in place_to_remove_info.incoming_transitions.iter() {
                let incoming_transition = petri_net.transitions.get_mut(*incoming_tx_id).unwrap();
                incoming_transition.postset_tokens.remove(&parallel_places.place2_id);    
            }
            // we remove the place from the preset of all outgoing transitions
            for (outgoing_tx_id,_) in place_to_remove_info.outgoing_transitions.iter() {
                let outgoing_transition = petri_net.transitions.get_mut(*outgoing_tx_id).unwrap();
                outgoing_transition.preset_tokens.remove(&parallel_places.place2_id);   
            }
        }

        // finally, remove the place from the net
        petri_net.remove_place(parallel_places.place2_id);
        petri_info.places_info.remove(parallel_places.place2_id);
        true 
    } else {
        false
    }
}

/// parallel places are pairs of places (p1,p2) such that:
/// - both p1 and p2 have the same place label
/// - p1 and p2 have the same number of tokens in the initial marking
/// - p1 and p2 have the same set of incoming transitions from which they take the same number of tokens
/// - p1 and p2 have the same set of outgoing transitions to which they give the same number of tokens
fn find_parallel_places(
    petri_net : &PetriNet, 
    petri_info : &PetriNetInfo,
    initial_markings : &Option<Marking>
) -> Option<ParallelPlacesPair> {
    'iter_pairs_of_places : for pl_pair in petri_info.places_info.iter().enumerate().combinations(2) {
        let (place_id1, place_info1) = pl_pair.first().unwrap();
        let (place_id2, place_info2) = pl_pair.get(1).unwrap();
        // p1 and p2 have the same set of incoming transitions from which they take the same number of tokens
        if place_info1.incoming_transitions != place_info2.incoming_transitions {
            continue 'iter_pairs_of_places;
        }
        // p1 and p2 have the same set of outgoing transitions to which they give the same number of tokens
        if place_info1.outgoing_transitions != place_info2.outgoing_transitions {
            continue 'iter_pairs_of_places;
        }
        // the two places must have the same place label
        let place_label1 = petri_net.places.get(*place_id1).unwrap();
        let place_label2 = petri_net.places.get(*place_id2).unwrap();
        if place_label1 != place_label2 {
            continue 'iter_pairs_of_places;
        }
        if let Some(im) = initial_markings {
            // p1 and p2 have the same number of tokens in the initial marking
            let toks_in1 = match im.tokens.get(place_id1) {
                None => {0},
                Some(x) => {*x}
            };
            let toks_in2 = match im.tokens.get(place_id2) {
                None => {0},
                Some(x) => {*x}
            };
            if toks_in1 != toks_in2 {
                continue 'iter_pairs_of_places;
            }
        }
        return Some(
            ParallelPlacesPair::new(
                *place_id1, 
                *place_id2
            )
        );
    }
    None 
}

