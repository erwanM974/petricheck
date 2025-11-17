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



use std::collections::HashMap;

use citreelo::{ctl::{BinaryCTLOperator, CTLFormula, CTLFormulaLeaf}};

use crate::{model::{net::PetriNet}, model_checking::props::{BuiltinPetriAtomicProposition, TokensCountAtom, TokensCountRelation}, util::{context::PetriNetContext, parse_ctl::error::PetriCtlParsingError}};


pub struct BuiltinPetriCtlParser {
    /// for each place name maps to its index in the PetriNet object
    pub place_name_to_index : HashMap<String,usize>,

    /// for each transition label maps to the index representing that label
    pub transition_label_to_label_id : HashMap<String,usize>,

    /// for each transition label
    /// the firing condition of a transition with that label 
    /// correspond to:
    /// ∨_{transitions with that label} 
    ///     ( 
    ///         ∧_{places that require tokens in the preset marking of the transition} 
    ///             (
    ///                 tokens-count(place) >= requirement  
    ///             )
    ///     )
    pub transition_label_to_firing_condition : HashMap<String,CTLFormula<BuiltinPetriAtomicProposition>>
}




impl BuiltinPetriCtlParser {
    pub fn from_context(context : &PetriNetContext, petri_net : &PetriNet) -> Result<Self,PetriCtlParsingError> {
        if petri_net.num_places != context.places_names.len() {
            return Err(PetriCtlParsingError::MismatchInTheNumberOfPlaces);
        }
        // ***
        let mut place_name_to_index = HashMap::new();
        for (place_id,place_name) in context.places_names.iter().enumerate() {
            if place_name_to_index.insert(place_name.clone(), place_id).is_some() {
                return Err(PetriCtlParsingError::MultiplePlacesHaveTheSameName);
            }
        }
        // ***
        if context.transitions_label_ids.len() != petri_net.transitions.len() {
            return Err(PetriCtlParsingError::MismatchInTheNumberOfTransitions);
        }
        // ***
        let mut transition_label_to_label_id = HashMap::new();
        for (tr_lab_id,tr_lab) in context.transition_labels.iter().enumerate() {
            if transition_label_to_label_id.insert(tr_lab.to_owned(), tr_lab_id).is_some() {
                return Err(PetriCtlParsingError::DuplicatedTransitionLabelInContext);
            }
        }
        // ***
        let mut transition_label_to_firing_condition = HashMap::new();
        for (tr_id, tr_label_id) in context.transitions_label_ids.iter().enumerate() {
            let transition = petri_net.transitions.get(tr_id).unwrap();
            let tr_firing_condition = {
                let mut ctl = CTLFormula::Leaf(CTLFormulaLeaf::True);
                for (place_id,req_num_toks) in transition.preset_tokens.tokens.iter().enumerate() {
                    if *req_num_toks > 0 {
                        let atom = BuiltinPetriAtomicProposition::TokensCount(
                            TokensCountRelation::GreaterOrEqual, 
                            TokensCountAtom::NumberOfTokensInPlace(place_id), 
                            TokensCountAtom::RawInteger(*req_num_toks)
                        );
                        ctl = CTLFormula::Binary(
                            BinaryCTLOperator::And, 
                            Box::new(ctl), 
                            Box::new(CTLFormula::Leaf(CTLFormulaLeaf::AtomicProp(atom)))
                        );
                    }
                }
                ctl
            };
            let tr_label = context.get_transition_label_from_label_id(*tr_label_id);
            // ***
            if let Some(ctl) = transition_label_to_firing_condition.remove(tr_label) {
                transition_label_to_firing_condition.insert(
                    tr_label.to_string(), 
                    CTLFormula::Binary(
                        BinaryCTLOperator::Or, 
                        Box::new(ctl), 
                        Box::new(tr_firing_condition)
                    )
                );
            } else {
                transition_label_to_firing_condition.insert(
                    tr_label.to_string(), 
                    tr_firing_condition
                );
            }
        }
        // ***
        Ok(Self {
            place_name_to_index,
            transition_label_to_label_id,
            transition_label_to_firing_condition
        })
    }
}


