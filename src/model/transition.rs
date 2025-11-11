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

use crate::model::marking::Marking;




#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct PetriTransition {
    pub preset_tokens : Marking,
    pub postset_tokens : Marking,
}

impl PetriTransition {
    pub fn new(preset_tokens: Marking, postset_tokens: Marking) -> Self {
        Self { preset_tokens, postset_tokens }
    }
    pub fn try_fire(&self, net_place_num : usize,from_marking : &Marking) -> Option<Marking> {
        let mut new_tokens = Vec::new();
        for place_id in 0..net_place_num {
            let toks_at_place_before_firing = from_marking.tokens.get(place_id).unwrap();
            let required_toks_at_place_to_fire = self.preset_tokens.tokens.get(place_id).unwrap();
            if required_toks_at_place_to_fire <= toks_at_place_before_firing {
                let added_toks_at_place_after_firing = self.postset_tokens.tokens.get(place_id).unwrap();
                new_tokens.push(toks_at_place_before_firing - required_toks_at_place_to_fire + added_toks_at_place_after_firing);
            } else {
                return None;
            }
        }
        Some(Marking::new(new_tokens))
    }
}
