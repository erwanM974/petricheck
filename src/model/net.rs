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

use std::rc::Rc;

use crate::model::{label::PetriStateLabel, transition::PetriTransition};




#[derive(Debug, Clone)]
pub struct PetriNet {
    pub places      : Vec<Option<Rc<PetriStateLabel>>>,
    pub transitions : Vec<PetriTransition> 
}

impl PetriNet {
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
}