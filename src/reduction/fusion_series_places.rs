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



pub struct SeriesPlacesPair {
    pub origin_place_id : usize,
    pub transition_id : usize,
    pub target_place_id : usize
}

impl SeriesPlacesPair {
    pub fn new(origin_place_id: usize, transition_id: usize, target_place_id: usize) -> Self {
        Self { origin_place_id, transition_id, target_place_id }
    }
}


pub fn find_and_simplify_series_places(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    if let Some(series_place) = find_series_places(&petri_net, petri_info) {
        // we will fuse the origin_place and the target_place so we need to update the initial_marking 
        if let Some(mark) = initial_markings {
            // if there are tokens in the origin_place, we transfer them to the target_place
            if let Some(toks_at_orig) = mark.tokens.remove(&series_place.origin_place_id) {
                match mark.tokens.get_mut(&series_place.target_place_id) {
                    Some(toks_at_target) => {
                        *toks_at_target = u32::max(toks_at_orig,*toks_at_target);
                    },
                    None => {
                        mark.tokens.insert(series_place.target_place_id,toks_at_orig);
                    }
                }
            }
            // and we remove the origin_place, shifting the indices 
            let mut new_tokens = BTreeMap::new();
            for (place_id,num_toks) in mark.tokens.iter() {
                match usize::cmp(&place_id,&series_place.origin_place_id) {
                    std::cmp::Ordering::Less => {
                        new_tokens.insert(*place_id,*num_toks);
                    },
                    std::cmp::Ordering::Equal => {
                        // we remove the origin_place
                    },
                    std::cmp::Ordering::Greater => {
                        new_tokens.insert(*place_id - 1,*num_toks);
                    }
                }
            }
            mark.tokens = new_tokens;
        }

        // remove the transition
        petri_net.transitions.remove(series_place.transition_id);
        petri_info.remove_transition(series_place.transition_id);
        // make all the transitions that target the origin_place instead target the target_place
        for transition in petri_net.transitions.iter_mut() {
            if let Some(toks_to_origin) = transition.postset_tokens.remove(&series_place.origin_place_id) {
                match transition.postset_tokens.get_mut(&series_place.target_place_id) {
                    Some(toks_to_target) => {
                        *toks_to_target += toks_to_origin;
                    },
                    None => {
                        transition.postset_tokens.insert(series_place.target_place_id,toks_to_origin);
                    },
                }
            }
        }
        let origin_incoming_transitions : Vec<(usize,u32)> = {
            let origin_place_info = petri_info.places_info.get_mut(series_place.origin_place_id).unwrap();
            origin_place_info.incoming_transitions.drain().collect()
        };
        let target_place_info = petri_info.places_info.get_mut(series_place.target_place_id).unwrap();
        for (place_id,toks) in origin_incoming_transitions {
            match target_place_info.incoming_transitions.get_mut(&place_id) {
                Some(x) => {
                    *x+=toks;
                },
                None => {
                    target_place_info.incoming_transitions.insert(place_id,toks);
                }
            }
        }
        // finally, we remove the origin place
        petri_net.remove_place(series_place.origin_place_id);
        petri_info.places_info.remove(series_place.origin_place_id);
        true 
    } else {
        false
    }
}

/// series places are pairs of place (p1,p2) such that:
/// - there exists a single transition t such that p1->t->p2
/// - and t is the only transition which accepts tokens from p1 
fn find_series_places(
    petri_net : &PetriNet, 
    petri_info : &PetriNetInfo
) -> Option<SeriesPlacesPair> {
    // find an origin place with only one outgoing transition
    for (origin_place_id,origin_place_info) in petri_info.places_info.iter().enumerate() {
        let origin_place = petri_net.places.get(origin_place_id).unwrap();
        if origin_place_info.outgoing_transitions.len() == 1 {
            let transition_id = origin_place_info.outgoing_transitions.keys().next().unwrap();
            let transition = petri_net.transitions.get(*transition_id).unwrap();
            // the outgoing transition must not have a label
            if transition.transition_label.is_none() {
                // the outgoing transition must have a single preset place and a single postset place
                if transition.number_of_preset_places() == 1 && transition.number_of_postset_places() == 1 {
                    let num_input_toks = transition.preset_tokens.get(&origin_place_id).unwrap();
                    let target_place_id = transition.postset_tokens.keys().next().unwrap();
                    let num_output_toks = transition.postset_tokens.get(target_place_id).unwrap();
                    // the transition must take and produce only 1 token
                    if *num_input_toks == 1 && *num_output_toks == 1 {
                        let target_place = petri_net.places.get(*target_place_id).unwrap();
                        // the origin and target places must have the same label
                        if origin_place == target_place {
                            return Some(SeriesPlacesPair::new(origin_place_id, *transition_id, *target_place_id));
                        }
                    }
                }
            }
        }
    }
    None 
}

