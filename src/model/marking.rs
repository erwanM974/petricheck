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


#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Marking {
    // place id (index in the vec) to number of tokens
    pub tokens : Vec<u32>
}

impl Marking {
    pub fn new(tokens: Vec<u32>) -> Self {
        Self { tokens }
    }
    pub fn new_empty(num_places : usize) -> Self {
        Self::new(
            vec![0; num_places]
        )
    }
}





