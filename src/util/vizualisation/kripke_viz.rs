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

use citreelo::util::viz_kripke::KripkeStructureGraphvizDrawer;
use itertools::Itertools;

use crate::{model::net::PetriNet, model_checking::state::PetriKripkeState};



pub struct PetriKripkeVisualizer<'a> {
    net : &'a PetriNet
}

impl<'a> PetriKripkeVisualizer<'a> {
    pub fn new(net: &'a PetriNet) -> Self {
        Self { net }
    }
}


impl<'a> KripkeStructureGraphvizDrawer<PetriKripkeState> for PetriKripkeVisualizer<'a> {
    fn get_doap_label(&self,doap : &PetriKripkeState) -> String {
        let toks_str = doap.marking.iter_tokens()
        .filter(|(_,y)| **y>0)
        .map(
            |(place_id,num_toks)| {
                let place = self.net.places.get(*place_id).unwrap();
                if let Some(place_label) = place {
                    format!("@p{}({:}):{}",place_id,place_label,num_toks)
                } else {
                    format!("@p{}:{}",place_id,num_toks)
                }
            }
        )
        .join("\n");
        match &doap.previous_tagged_transition_label {
            Some(previous_transition_label) => {
                format!("{}\nprev:{}",toks_str,previous_transition_label)
            },
            None => {
                toks_str
            }
        }
    }
}

