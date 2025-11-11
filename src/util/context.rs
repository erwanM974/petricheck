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



pub struct PetriNetContext {
    // names of each place, index is id of the place
    pub(crate) places_names : Vec<String>,
    // ids of the labels for each transition, index is the id of the transition
    pub(crate) transitions_label_ids : Vec<usize>,
    // names of each transition label, index is transition label id
    pub(crate) transition_labels : Vec<String>
}

impl PetriNetContext {
    pub fn new(places_names: Vec<String>, transitions_label_ids: Vec<usize>, transition_labels: Vec<String>) -> Self {
        Self { places_names, transitions_label_ids, transition_labels }
    }

    pub fn get_transition_label_from_transition_id(&self, tr_id : usize) -> &str {
        let lab_id = self.transitions_label_ids.get(tr_id).unwrap();
        self.transition_labels.get(*lab_id).unwrap()
    }

    pub fn get_transition_label_from_label_id(&self, lab_id : usize) -> &str {
        self.transition_labels.get(lab_id).unwrap()
    }

    pub fn get_place_label(&self, place_id : usize) -> &str {
        self.places_names.get(place_id).unwrap()
    }
}

