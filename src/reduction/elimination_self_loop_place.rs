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


use crate::{model::{marking::Marking, net::PetriNet}, reduction::info::PetriNetInfo};



pub struct SelfLoopPlace {
    pub place_id : usize
}

impl SelfLoopPlace {
    pub fn new(place_id: usize) -> Self {
        Self { place_id }
    }
}



pub fn find_and_simplify_self_loop_place(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    // there must be an initial marking to perform this reduction
    if let Some(im) = initial_markings {
        if let Some(self_loop_place) = find_self_loop_place(petri_net, petri_info, im) {
            // we remove the self_loop_place from the initial marking
            // we remove the self_loop_place from the preset and postset of all associated transitions
            // we remove the self_loop_place from the net
            {
                // we remove the place from the initial marking, shifting the indices
                let mut new_tokens = BTreeMap::new();
                for (place_id,num_toks) in im.tokens.iter() {
                    match usize::cmp(place_id,&self_loop_place.place_id) {
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
                im.tokens = new_tokens;
            }

            // we remove the self_loop_place from the preset and postset of all associated transitions
            {
                let place_info = petri_info.places_info.get(self_loop_place.place_id).unwrap();
                for tr_id in place_info.incoming_transitions.keys() {
                    let transition = petri_net.transitions.get_mut(*tr_id).unwrap();
                    // remove the place from the preset and postset of the transition 
                    transition.preset_tokens.remove(&self_loop_place.place_id);
                    transition.postset_tokens.remove(&self_loop_place.place_id);
                }
            };
            
            // we remove the self_loop_place from the net
            petri_net.remove_place(self_loop_place.place_id);
            petri_info.places_info.remove(self_loop_place.place_id);

            return true;
        }
    }
    false
}

/// a self-loop place is a place p such that:
/// - it has the empty place label
/// - its map of outgoing transitions to the number of tokens they take is the same as 
///   its map of incoming transitions to the number of tokens they give
/// - the number of tokens in p in the initial marking is greater that the maximum number of tokens taken by any of its outgoing transitions
/// - let's remark that it also eliminate dead places (i.e. no outgoing nor incoming transitions)
fn find_self_loop_place(
    petri_net : &PetriNet, 
    petri_info : &PetriNetInfo,
    initial_markings : &Marking
) -> Option<SelfLoopPlace> {
    'iter_places : for (place_id,place_info) in petri_info.places_info.iter().enumerate() {
        // the place must have the same maps of incoming and outgoing transitions
        if place_info.outgoing_transitions != place_info.incoming_transitions {
            continue 'iter_places;
        }
        // the place, that will be deleted, must contain the empty place label
        let place_label = petri_net.places.get(place_id).unwrap();
        if place_label.is_some() {
            continue 'iter_places;
        }
        match place_info.outgoing_transitions.values().max() {
            None => {
                // the place is dead, i.e. has no outgoing nor incoming transitions
                return Some(SelfLoopPlace::new(place_id));
            },
            Some(max_taken_num_toks) => {
                if let Some(num_toks_in_place) = initial_markings.get_num_toks_at_place(&place_id) {
                    if num_toks_in_place >= max_taken_num_toks {
                        return Some(SelfLoopPlace::new(place_id));
                    }
                }
            }
        }
        
    }
    None 
}

