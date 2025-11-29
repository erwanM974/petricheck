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

use citreelo::{ctl::{BinaryCTLOperator, CTLFormula, CTLFormulaLeaf}};

use crate::{model::{label::PetriTransitionLabel, net::PetriNet}, model_checking::props::{BuiltinPetriAtomicProposition, TokensCountAtom, TokensCountRelation}, util::parse_ctl::error::PetriCtlParsingError};


pub struct BuiltinPetriCtlParser {
    /// for each place name maps to its index in the PetriNet object
    pub place_name_to_index : HashMap<String,usize>,

    /// for each transition label maps to the index representing the tag
    /// that decorates certain states to signifies that the previous transition 
    pub transition_label_to_ref : HashMap<String,Rc<PetriTransitionLabel>>,

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
    pub fn from_net
    (
        petri_net : &PetriNet
    ) -> Result<Self,PetriCtlParsingError> {
        // ***
        let mut place_name_to_index = HashMap::new();
        for (place_id,place_content) in petri_net.places.iter().enumerate() {
            if let Some(place_lab_ref) = place_content {
                if place_name_to_index.insert(place_lab_ref.label.to_string(), place_id).is_some() {
                    return Err(PetriCtlParsingError::MultiplePlacesHaveTheSameName);
                }
            }
        }
        // ***
        let mut transition_label_to_ref = HashMap::new();
        for transition in &petri_net.transitions {
            if let Some(transition_label_ref) = &transition.transition_label {
                if let Some(other_ref) = transition_label_to_ref.insert(
                    transition_label_ref.label.to_owned(), 
                    transition_label_ref.clone()
                ) {
                    if other_ref != *transition_label_ref {
                        return Err(PetriCtlParsingError::DuplicatedTransitionLabelInContext);
                    }
                }
            }
        }
        // ***
        let mut transition_label_to_firing_condition = HashMap::new();
        for transition in &petri_net.transitions {
            if let Some(transition_label_ref) = &transition.transition_label {
                let tr_firing_condition = {
                    let mut ctl = CTLFormula::Leaf(CTLFormulaLeaf::True);
                    for (place_id,req_num_toks) in transition.iter_preset_tokens() {
                        debug_assert!(*req_num_toks > 0);
                        let atom = BuiltinPetriAtomicProposition::TokensCount(
                            TokensCountRelation::GreaterOrEqual, 
                            TokensCountAtom::NumberOfTokensInPlace(*place_id), 
                            TokensCountAtom::RawInteger(*req_num_toks)
                        );
                        ctl = CTLFormula::Binary(
                            BinaryCTLOperator::And, 
                            Box::new(ctl), 
                            Box::new(CTLFormula::Leaf(CTLFormulaLeaf::AtomicProp(atom)))
                        );
                    }
                    ctl
                };
                // ***
                if let Some(ctl) = transition_label_to_firing_condition.remove(&transition_label_ref.label) {
                    transition_label_to_firing_condition.insert(
                        transition_label_ref.label.to_string(), 
                        CTLFormula::Binary(
                            BinaryCTLOperator::Or, 
                            Box::new(ctl), 
                            Box::new(tr_firing_condition)
                        )
                    );
                } else {
                    transition_label_to_firing_condition.insert(
                        transition_label_ref.label.to_string(), 
                        tr_firing_condition
                    );
                }
            }
        }
        // ***
        Ok(Self {
            place_name_to_index,
            transition_label_to_ref,
            transition_label_to_firing_condition
        })
    }
}


