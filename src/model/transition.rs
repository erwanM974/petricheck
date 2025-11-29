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

use std::collections::{BTreeMap, HashMap};

use crate::model::marking::Marking;




#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PetriTransition {
    preset_tokens : HashMap<usize,u32>,
    postset_tokens : HashMap<usize,u32>,
}

impl PetriTransition {
    pub fn new(preset_tokens: HashMap<usize,u32>, postset_tokens: HashMap<usize,u32>) -> Self {
        Self { preset_tokens, postset_tokens }
    }

    pub fn number_of_preset_places(&self) -> usize {
        self.preset_tokens.keys().len()
    }

    pub fn number_of_postset_places(&self) -> usize {
        self.postset_tokens.keys().len()
    }
    
    pub fn iter_preset_tokens(&self) -> impl Iterator<Item=(&usize,&u32)> {
        self.preset_tokens.iter()
    }
    
    pub fn iter_postset_tokens(&self) -> impl Iterator<Item=(&usize,&u32)> {
        self.postset_tokens.iter()
    }

    pub fn unwrap(self) -> (impl Iterator<Item=(usize,u32)>,impl Iterator<Item=(usize,u32)>) {
        (self.preset_tokens.into_iter(),self.postset_tokens.into_iter())
    }

    pub fn try_fire(&self, net_place_num : usize,from_marking : &Marking) -> Option<Marking> {
        let mut new_tokens = BTreeMap::new();
        for place_id in 0..net_place_num {
            let toks_at_place_before_firing = match from_marking.get_num_toks_at_place(&place_id) {
                Some(toks) => {*toks},
                None => {0}
            };
            let required_toks_at_place_to_fire = match self.preset_tokens.get(&place_id) {
                Some(toks) => {*toks},
                None => {0}
            };
            if required_toks_at_place_to_fire <= toks_at_place_before_firing {
                let added_toks_at_place_after_firing = match self.postset_tokens.get(&place_id) {
                    Some(toks) => {*toks},
                    None => {0}
                };
                let remaining_tokens = toks_at_place_before_firing - required_toks_at_place_to_fire + added_toks_at_place_after_firing;
                if remaining_tokens > 0 {
                    new_tokens.insert(place_id,remaining_tokens);
                }
            } else {
                return None;
            }
        }
        Some(Marking::new(new_tokens))
    }
}
