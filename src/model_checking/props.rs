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

use citreelo::kripke::AtomicProposition;

use crate::{model::marking::Marking, model_checking::state::PetriKripkeState};



#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub enum TokensCountAtom {
    RawInteger(u32),
    NumberOfTokensInPlace(usize)
}

impl TokensCountAtom {
    pub fn interpret_as_u32(&self, marking : &Marking) -> u32 {
        match self {
            TokensCountAtom::RawInteger(raw_int) => {*raw_int},
            TokensCountAtom::NumberOfTokensInPlace(place_id) => {
                *marking.tokens.get(*place_id).unwrap()
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub enum TokensCountRelation {
    StrictlyGreater,
    GreaterOrEqual,
    Equal,
    LowerOrEqual,
    StrictlyLower
}

impl TokensCountRelation {
    pub fn eval(&self,left : u32,right:u32)-> bool {
        match self {
            TokensCountRelation::StrictlyGreater => left > right,
            TokensCountRelation::GreaterOrEqual  => left >= right,
            TokensCountRelation::Equal           => left == right,
            TokensCountRelation::LowerOrEqual    => left <= right,
            TokensCountRelation::StrictlyLower   => left < right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub enum BuiltinPetriAtomicProposition {
    TokensCount(TokensCountRelation, TokensCountAtom, TokensCountAtom),
    PreviousTransitionLabelIdMustBe(usize)
}



impl AtomicProposition<PetriKripkeState> for BuiltinPetriAtomicProposition {
    fn is_satisfied_on_state_domain(&self, state_domain : &PetriKripkeState) -> bool {
        match self {
            BuiltinPetriAtomicProposition::TokensCount(rel, left, right) => {
                let left_int = left.interpret_as_u32(&state_domain.marking);
                let right_int = right.interpret_as_u32(&state_domain.marking);
                rel.eval(left_int,right_int)
            },
            BuiltinPetriAtomicProposition::PreviousTransitionLabelIdMustBe(lab_id) => {
                match &state_domain.previous_transition_label_id {
                    Some(got_lab_id) => got_lab_id == lab_id,
                    None => false,
                }
            },
        }
    }
}