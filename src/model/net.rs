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

use std::{collections::HashMap, rc::Rc};

use crate::model::{label::{PetriStateLabel, PetriTransitionLabel}, transition::{self, PetriTransition}};




#[derive(Debug, Clone)]
pub struct PetriNet {
    pub places      : Vec<Option<Rc<PetriStateLabel>>>,
    pub transitions : Vec<PetriTransition> 
}

impl PetriNet {

    pub fn remove_place(&mut self, place_to_remove_id : usize) {
        for transition in self.transitions.iter_mut() {
            transition.remove_place(place_to_remove_id);
        }
        self.places.remove(place_to_remove_id);
    }

    pub fn new(places: Vec<Option<Rc<PetriStateLabel>>>, transitions: Vec<PetriTransition>) -> Self {
        Self { places, transitions }
    }
    
    pub fn new_empty() -> Self {
        Self { places:Vec::new(), transitions:Vec::new() }
    }

    pub fn add_place(&mut self, place_label : Option<Rc<PetriStateLabel>>) -> usize {
        let state_id = self.places.len();
        self.places.push(place_label);
        state_id
    }

    pub fn add_transition(&mut self, transition : PetriTransition) -> usize {
        let tr_id = self.transitions.len();
        self.transitions.push(transition);
        tr_id
    }

    pub fn relabel_places(&mut self, relabbelling : HashMap<PetriStateLabel, Option<Rc<PetriStateLabel>>>) {
        let mut new_places = vec![];
        for place in self.places.drain(..) {
            let mut replaced = false;
            if let Some(x) = &place {
                if let Some(new_lab) = relabbelling.get(x) {
                    new_places.push(new_lab.clone());
                    replaced = true;
                }
            }
            if !replaced {
                new_places.push(place);
            }
        }
        self.places = new_places;
    }

    pub fn relabel_transitions(&mut self, relabbelling : HashMap<PetriTransitionLabel, Option<Rc<PetriTransitionLabel>>>) {
        let mut new_transitions = vec![];
        for transition in self.transitions.drain(..) {
            if let Some(x) = &transition.transition_label {
                if let Some(new_lab) = relabbelling.get(x) {
                    let new_transition = PetriTransition::new(
                        new_lab.clone(), 
                        transition.preset_tokens, 
                        transition.postset_tokens
                    );
                    new_transitions.push(new_transition);
                } else {
                    new_transitions.push(transition);
                }
            } else {
                new_transitions.push(transition);
            }
        }
        self.transitions = new_transitions;
    }

}