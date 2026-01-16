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

use petricheck::model::label::{PetriStateLabel, PetriTransitionLabel};
use petricheck::model::net::PetriNet;
use petricheck::model::transition::PetriTransition;
use petricheck::model_checking::props::{BuiltinPetriAtomicProposition, TokensCountAtom, TokensCountRelation};
use petricheck::util::parse_ctl::parser::BuiltinPetriCtlParser;
use petricheck::util::vizualisation::petri_viz::petri_repr;
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::hash_map;

use citreelo::ctl::{BinaryCTLOperator, CTLFormula, CTLFormulaLeaf};

use citreelo::parser::CtlFormulaParser;






fn tool_test_parse_ctl(
    title : &str, 
    pn : PetriNet,
    formulaes : HashMap< &str, CTLFormula<BuiltinPetriAtomicProposition> >
) {
    let folder_name = "test_pase_ctl";
    let _ = std::fs::create_dir(folder_name);
    {
        let gv = petri_repr(&pn,&None);
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}/{}", folder_name, title), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    let ctl_parser = BuiltinPetriCtlParser::from_net(&pn).unwrap();
    for (phi_as_str,expected_formula) in formulaes {
        let (_,phi) = ctl_parser.parse_ctl_formula::<nom::error::Error<&str>>(phi_as_str).unwrap();
        assert_eq!(phi,expected_formula,"{} : {}", title, phi_as_str);
    }
}



#[test]
pub fn test_tokens_count() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let tr_b = Rc::new(PetriTransitionLabel::new("B".to_string()));
    let tr_c = Rc::new(PetriTransitionLabel::new("C".to_string()));
    let mut pn = PetriNet::new(
        vec![
            Some(Rc::new(PetriStateLabel::new("P1".to_string()))),
            Some(Rc::new(PetriStateLabel::new("P2".to_string()))),
            Some(Rc::new(PetriStateLabel::new("P3".to_string()))),
            Some(Rc::new(PetriStateLabel::new("P4".to_string()))),
        ], 
        vec![
            PetriTransition::new(
                Some(tr_a.clone()),
                hash_map! {0=>1},
                hash_map! {1=>1}
            ),
            PetriTransition::new(
                Some(tr_b.clone()),
                hash_map! {0=>1},
                hash_map! {2=>1,3=>1}
            ),
            PetriTransition::new(
                Some(tr_c.clone()),
                hash_map! {2=>1,3=>1},
                hash_map! {1=>1}
            )
        ]
    );
    let one_in_p1 = CTLFormula::Leaf(
        CTLFormulaLeaf::AtomicProp(
            BuiltinPetriAtomicProposition::TokensCount(
                TokensCountRelation::GreaterOrEqual,
                TokensCountAtom::NumberOfTokensInPlace(0),
                TokensCountAtom::RawInteger(1)
            )
        )
    );
    let one_in_p2 = CTLFormula::Leaf(
        CTLFormulaLeaf::AtomicProp(
            BuiltinPetriAtomicProposition::TokensCount(
                TokensCountRelation::GreaterOrEqual,
                TokensCountAtom::NumberOfTokensInPlace(1),
                TokensCountAtom::RawInteger(1)
            )
        )
    );
    let one_in_p3 = CTLFormula::Leaf(
        CTLFormulaLeaf::AtomicProp(
            BuiltinPetriAtomicProposition::TokensCount(
                TokensCountRelation::GreaterOrEqual,
                TokensCountAtom::NumberOfTokensInPlace(2),
                TokensCountAtom::RawInteger(1)
            )
        )
    );
    let one_in_p4 = CTLFormula::Leaf(
        CTLFormulaLeaf::AtomicProp(
            BuiltinPetriAtomicProposition::TokensCount(
                TokensCountRelation::GreaterOrEqual,
                TokensCountAtom::NumberOfTokensInPlace(3),
                TokensCountAtom::RawInteger(1)
            )
        )
    );
    let fireable_a = one_in_p1.clone();
    let fireable_b = one_in_p1.clone();
    let fireable_c = CTLFormula::Binary(
        BinaryCTLOperator::And, 
        Box::new(one_in_p3.clone()),
        Box::new(one_in_p4.clone())
    );
    let formulaes = hash_map! {
        "tokens-count(\"P1\")>=1" => one_in_p1.clone(),
        "tokens-count(\"P2\")>=1" => one_in_p2.clone(),
        "tokens-count(\"P3\")>=1" => one_in_p3.clone(),
        "tokens-count(\"P4\")>=1" => one_in_p4.clone(),
        "is-fireable(\"A\")" => fireable_a.clone(),
        "is-fireable(\"B\")" => fireable_b.clone(),
        "is-fireable(\"C\")" => fireable_c.clone(),
    };
    tool_test_parse_ctl(
        "tokenscount1",
        pn.clone(),
        formulaes
    );
    let relabelling = hash_map! {(*tr_c).clone()=>Some(tr_a.clone())};
    let fireable_a_after_relabelling = CTLFormula::Binary(
        BinaryCTLOperator::Or, 
        Box::new(fireable_a),
        Box::new(fireable_c)
    );
    let formulaes_after_relabelling = hash_map! {
        "is-fireable(\"A\")" => fireable_a_after_relabelling,
        "is-fireable(\"B\")" => fireable_b,
        "is-fireable(\"C\")" => CTLFormula::Leaf(CTLFormulaLeaf::False),
    };
    pn.relabel_transitions(relabelling);
    tool_test_parse_ctl(
        "tokenscount2",
        pn.clone(),
        formulaes_after_relabelling
    );
}