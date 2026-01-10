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



pub struct SeriesTransitionsPairVariant1 {
    pub preceding_transition_id : usize,
    pub place_id : usize,
    pub succeeding_transition_id : usize
}

impl SeriesTransitionsPairVariant1 {
    pub fn new(preceding_transition_id: usize, place_id: usize, succeeding_transition_id: usize) -> Self {
        Self { preceding_transition_id, place_id, succeeding_transition_id }
    }
}


pub fn find_and_simplify_series_transitions_variant1(
    petri_net : &mut PetriNet,
    petri_info : &mut PetriNetInfo,
    initial_markings : &mut Option<Marking>
) -> bool {
    if let Some(mut series_transitions) = find_series_transitions_variant1(petri_net, petri_info, initial_markings) {
        // we delete the intermediate place and the succeeding transition
        // and we add the target places of the succeeding transition to the target places of the preceding transition

        if let Some(mark) = initial_markings {
            // we remove the place, shifting the indices
            let mut new_tokens = BTreeMap::new();
            for (place_id,num_toks) in mark.tokens.iter() {
                match usize::cmp(place_id,&series_transitions.place_id) {
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

        // remove the succeeding transition
        let succeeding_transition = petri_net.transitions.remove(series_transitions.succeeding_transition_id);
        petri_info.remove_transition(series_transitions.succeeding_transition_id);
        if series_transitions.succeeding_transition_id < series_transitions.preceding_transition_id {
            series_transitions.preceding_transition_id -= 1;
        }

        let preceding_transition = petri_net.transitions.get_mut(series_transitions.preceding_transition_id).unwrap();
        // remove the place from the postset of the preceeding transition 
        preceding_transition.postset_tokens.remove(&series_transitions.place_id);
        // add to the preceeding transition postset all the places that the succeeding transition targets
        for (target_place_id,num_toks) in succeeding_transition.iter_postset_tokens() {
            preceding_transition.postset_tokens.insert(*target_place_id,*num_toks);
            let target_place_info = petri_info.places_info.get_mut(*target_place_id).unwrap();
            target_place_info.incoming_transitions.insert(series_transitions.preceding_transition_id, *num_toks);
        }
        
        // finally, remove the place
        petri_net.remove_place(series_transitions.place_id);
        petri_info.places_info.remove(series_transitions.place_id);
        true 
    } else {
        false
    }
}

/// series transitions variant 1 are pairs of transitions (t1,t2) such that:
/// - there exists a single place p such that t1->p->t2
/// - t2 has the empty label
/// - t2 only accepts tokens from p
/// - p only accepts tokens from t1
/// - p only feeds tokens to t2
fn find_series_transitions_variant1(
    petri_net : &PetriNet, 
    petri_info : &PetriNetInfo,
    initial_markings : &Option<Marking>
) -> Option<SeriesTransitionsPairVariant1> {
    'iter_places : for (place_id,place_info) in petri_info.places_info.iter().enumerate() {
        // find a place with only one incoming transition and one outgoing transition
        if (place_info.outgoing_transitions.len() == 1) && (place_info.incoming_transitions.len() == 1) {
            // as the place, if it matches all requirements, will be deleted, it must not contain tokens in the initial marking 
            if let Some(marks) = initial_markings {
                if let Some(num_toks) = marks.get_num_toks_at_place(&place_id) {
                    if *num_toks > 0 {
                        continue 'iter_places;
                    }
                }
            }
            let outgoing_transition_id = place_info.outgoing_transitions.keys().next().unwrap();
            let outgoing_transition = petri_net.transitions.get(*outgoing_transition_id).unwrap();
            // the outgoing transition must have no label and must only accept tokens from the place
            if (outgoing_transition.transition_label.is_none()) && (outgoing_transition.preset_tokens.len() == 1) {
                let incoming_transition_id = place_info.incoming_transitions.keys().next().unwrap();
                let incoming_transition = petri_net.transitions.get(*incoming_transition_id).unwrap();
                {
                    let num_toks_to_t2 = outgoing_transition.preset_tokens.get(&place_id).unwrap();
                    // the incoming transition must put the correct number of tokens in p 
                    let num_toks_from_t1 = incoming_transition.postset_tokens.get(&place_id).unwrap();
                    if num_toks_from_t1 != num_toks_to_t2 {
                        continue 'iter_places;
                    }
                }
                {
                    // the place, that will be deleted, must contain the empty label
                    let place_label = petri_net.places.get(place_id).unwrap();
                    if place_label.is_some() {
                        continue 'iter_places;
                    }
                }
                // if the outgoing transition target places that are also targetted by the incoming transition
                // then both transitions must put the same number of tokens in these places
                for (target_place_id,num_toks_from_t2) in outgoing_transition.iter_postset_tokens() {
                    if let Some(num_toks_from_t1) = incoming_transition.postset_tokens.get(target_place_id) {
                        if num_toks_from_t1 != num_toks_from_t2 {
                            continue 'iter_places;
                        }
                    }
                }
                return Some(
                    SeriesTransitionsPairVariant1::new(
                        *incoming_transition_id,
                        place_id, 
                        *outgoing_transition_id
                    )
                );
            }
        }
    }
    None 
}

