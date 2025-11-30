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

use citreelo::{ctl::{CTLFormula, CTLFormulaLeaf}, parser::CtlFormulaParser};
use nom::{Parser, branch::alt, bytes::complete::{tag, take_while}, character::complete::{alpha1, digit1}, combinator::{map, value}, error::ErrorKind, sequence::delimited};

use crate::{model_checking::props::{BuiltinPetriAtomicProposition, TokensCountAtom, TokensCountRelation}, util::parse_ctl::parser::BuiltinPetriCtlParser};




impl CtlFormulaParser<BuiltinPetriAtomicProposition> for BuiltinPetriCtlParser {
    fn parse_atomic_proposition<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, CTLFormula<BuiltinPetriAtomicProposition>, E> {
        alt((
            map(
                |x|self.parse_token_count_requirement(x),
                |y| CTLFormula::Leaf(CTLFormulaLeaf::AtomicProp(y))
            ),
            map(
                |x|self.parse_previous_label_condition(x),
                CTLFormula::Leaf
            ),
            |x| self.parse_transition_label_firing_condition(x),
        )).parse(input)
    }
}




fn parse_u32<'a, E: nom::error::ParseError<&'a str>>(
    input : &'a str
) -> nom::IResult<&'a str, u32, E> {
    let mut parser = digit1::<&'a str,E>;
    match parser.parse(input) {
        Ok((rem,raw_int_str)) => {
            match str::parse::<u32>(raw_int_str) {
                Ok(raw_int) => {
                    Ok((rem,raw_int))
                }
                Err(_) => {
                    Err(nom::Err::Error(nom::error::make_error(input, ErrorKind::Fail)))
                }
            }
        },
        Err(_) => {
            Err(nom::Err::Error(nom::error::make_error(input, ErrorKind::Fail)))
        },
    }
}

fn parse_petri_element_reference<'a,E: nom::error::ParseError<&'a str>>(input : &'a str) -> nom::IResult<&'a str, String,E> {
    let mut parser = delimited(
        tag("\""),
        (
            alpha1,
            take_while(|c: char| c == '_' || c.is_alphanumeric())
        ),
        tag("\"")
    );
    parser.parse(input).map(|(rem, (x, y))| {
        (rem, format!("{}{}", x, y))
    })
}





impl BuiltinPetriCtlParser {
    
    fn parse_place_name_into_place_id<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, usize, E> {
        match parse_petri_element_reference(input) {
            Err(e) => {
                Err(e)
            },
            Ok((rem,lab)) => {
                match self.place_name_to_index.get(&lab) {
                    None => {
                        Err(nom::Err::Error(nom::error::make_error(input, ErrorKind::Fail)))
                    }
                    Some(index) => {
                        Ok((rem,*index))
                    }
                }
            }
        }
    }

    fn parse_token_count_atom<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, TokensCountAtom, E> {
        alt((
            map(
                delimited(
                    tag("tokens-count("), 
                    |x| self.parse_place_name_into_place_id(x), 
                    tag(")")
                ),
                TokensCountAtom::NumberOfTokensInPlace
            ),
            map(
                parse_u32,
                TokensCountAtom::RawInteger
            )
        )).parse(input)
    }

    

    fn parse_token_count_requirement<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, BuiltinPetriAtomicProposition, E> {
        map(
            (
                |x| self.parse_token_count_atom(x),
                alt((
                    value(TokensCountRelation::LowerOrEqual, tag("<=")),
                    value(TokensCountRelation::StrictlyLower, tag("<")),
                    value(TokensCountRelation::Equal, tag("=")),
                    value(TokensCountRelation::GreaterOrEqual, tag(">=")),
                    value(TokensCountRelation::StrictlyGreater, tag(">")),
                )),
                |x| self.parse_token_count_atom(x),
            ),
            |(left,op,right)| BuiltinPetriAtomicProposition::TokensCount(op,left,right)
        ).parse(input)
    }

    fn parse_transition_label_firing_condition<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, CTLFormula<BuiltinPetriAtomicProposition>, E> {
        let mut parser = delimited(
            tag("is-fireable("), 
            |x| parse_petri_element_reference::<'a,E>(x), 
            tag(")")
        );
        match parser.parse(input) {
            Ok((rem,tr_lab)) => {
                match self.transition_label_to_firing_condition.get(&tr_lab) {
                    Some(firing_cond) => {
                        Ok((rem,firing_cond.clone()))
                    },
                    None => {
                        // an unknown transition is not fireable
                        Ok((rem,CTLFormula::Leaf(CTLFormulaLeaf::False)))
                    },
                }
            },
            Err(e) => Err(e),
        }
    }


    fn parse_previous_label_condition<'a, E: nom::error::ParseError<&'a str>>(
        &self,
        input : &'a str
    ) -> nom::IResult<&'a str, CTLFormulaLeaf<BuiltinPetriAtomicProposition>, E> {
        let mut parser = delimited(
            tag("is-previous("), 
            |x| parse_petri_element_reference::<'a,E>(x), 
            tag(")")
        );
        match parser.parse(input) {
            Ok((rem,tr_lab)) => {
                match self.transition_label_to_ref.get(&tr_lab) {
                    Some(label_id) => {
                        let prop = BuiltinPetriAtomicProposition::PreviousTransitionLabelMustBe(label_id.clone());
                        Ok((rem,CTLFormulaLeaf::AtomicProp(prop)))
                    },
                    None => {
                        // an unknown transition is not previous
                        Ok((rem,CTLFormulaLeaf::False))
                    },
                }
            },
            Err(e) => Err(e),
        }
    }

}


