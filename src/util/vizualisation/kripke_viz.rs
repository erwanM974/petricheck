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

use crate::model_checking::state::PetriKripkeState;




pub trait PetriKripkeVisualizer {

    fn get_transition_label_from_label_id(&self, lab_id : &usize) -> &str;

    fn get_place_label(&self, place_id : &usize) -> &str;

    fn get_doap_label(&self,doap : &PetriKripkeState) -> String {
        let toks_str = doap.marking.iter_tokens()
        .filter(|(_,y)| **y>0)
        .map(|(place_id,num_toks)| format!("@({}:{})",self.get_place_label(place_id),num_toks))
        .join("\n");
        match &doap.previous_transition_label_id {
            Some(lab_id) => {
                format!("{}\nprev:{}",toks_str,self.get_transition_label_from_label_id(lab_id))
            },
            None => {
                toks_str
            }
        }
    }

}