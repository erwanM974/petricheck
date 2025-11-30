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


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Marking {
    // place id to number of tokens
    // use BTreeMap so one can derive the Hash of Marking
    pub(crate) tokens : BTreeMap<usize,u32>
}

impl Marking {
    pub fn new(tokens: BTreeMap<usize,u32>) -> Self {
        Self { tokens }
    }
    pub fn get_num_toks_at_place(&self, place_id : &usize) -> Option<&u32> {
        self.tokens.get(place_id)
    }
    pub fn iter_tokens(&self) -> impl Iterator<Item=(&usize,&u32)> {
        self.tokens.iter()
    }
}





