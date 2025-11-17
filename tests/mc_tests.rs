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

use petricheck::{model::{marking::Marking, net::PetriNet, transition::PetriTransition}, model_checking::to_kripke::{DefaultPetriKripkeStateProducer, petri_to_kripke}, util::{context::PetriNetContext, parse_ctl::parser::BuiltinPetriCtlParser, vizualisation::petri_viz::PetriNetVisualizer}};
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::{hash_map, hash_set};

use citreelo::util::viz_kripke::KripkeStructureGraphvizDrawer;

use citreelo::solve::is_ctl_formula_sat;
use citreelo::parser::CtlFormulaParser;


#[test]
pub fn test_simple_example() {
    let petri_net = PetriNet::new(
        5, 
        vec![
            PetriTransition::new(Marking::new(vec![1,0,1,0,0]), Marking::new(vec![0,1,0,0,0])),
            PetriTransition::new(Marking::new(vec![0,0,1,1,0]), Marking::new(vec![0,0,0,0,1])),
            PetriTransition::new(Marking::new(vec![0,1,0,0,0]), Marking::new(vec![1,0,1,0,0])),
            PetriTransition::new(Marking::new(vec![0,0,0,0,1]), Marking::new(vec![0,0,1,1,0])),
        ]
    );
    let initial_marking = Marking::new(vec![1,0,1,1,0]);
    // ***
    let context = PetriNetContext::new(
        vec![
            "A_U".to_string(),
            "A_L".to_string(),
            "CTL".to_string(),
            "B_U".to_string(),
            "B_L".to_string(),
        ],
        vec![
            0,
            0,
            1,
            1
        ],
        vec![
            "lock".to_string(),
            "unlock".to_string(),
        ]
    );
    {
        let gv = context.petri_repr(&petri_net,Some(&initial_marking));
        gv.print_dot(&[".".to_string()], "lock_petri", &GraphVizOutputFormat::png).unwrap();
    }
    // ***
    let kripke_only_marks = petri_to_kripke(
        &petri_net,
        initial_marking.clone(),
        DefaultPetriKripkeStateProducer::new(hash_map! {})
    );
    {
        let gv = context.get_kripke_repr(&kripke_only_marks);
        gv.print_dot(&[".".to_string()], "lock_kripke1", &GraphVizOutputFormat::png).unwrap();
    }
    // ***
    let kripke_with_prev_labs = petri_to_kripke(
        &petri_net,
        initial_marking.clone(),
        DefaultPetriKripkeStateProducer::new(hash_map! {0=>0,1=>0,2=>1,3=>1})
    );
    {
        let gv = context.get_kripke_repr(&kripke_with_prev_labs);
        gv.print_dot(&[".".to_string()], "lock_kripke2", &GraphVizOutputFormat::png).unwrap();
    }
    // ***
    let initial_states = hash_set! {0};
    let ctl_parser = BuiltinPetriCtlParser::from_context(&context, &petri_net).unwrap();
    
    let phis = vec![
        // at initial state
        (r#"tokens-count("A_L")=0"# , true),  
        (r#"tokens-count("B_L")=0"# , true),  
        (r#"tokens-count("A_L")>0"#  , false),  
        (r#"tokens-count("B_L")>0"#  , false),  
        (r#"tokens-count("A_U")=1"# , true),  
        (r#"tokens-count("B_U")=1"# , true),  
        (r#"tokens-count("A_U")=0"# , false),  
        (r#"tokens-count("B_U")=0"# , false),  
        (r#"tokens-count("CTL")=1"# , true),  
        (r#"tokens-count("CTL")=0"# , false),  
        // safety properties
        (r#"A(G(!((tokens-count("A_U")>0)&(tokens-count("B_U")>0))))"# , false),  
        (r#"A(G(!((tokens-count("A_L")>0)&(tokens-count("B_L")>0))))"# , true),  
        (r#"A(G( ((tokens-count("A_L")=0)&(tokens-count("A_U")=1)) | ((tokens-count("A_L")=1)&(tokens-count("A_U")=0)) ))"# , true),  
        (r#"A(G( ((tokens-count("B_L")=0)&(tokens-count("B_U")=1)) | ((tokens-count("B_L")=1)&(tokens-count("B_U")=0)) ))"# , true),  
        (r#"A(G( (tokens-count("CTL")=0) => (!( is-fireable("lock") )) ))"# , true),  
        (r#"A(G( (tokens-count("CTL")=0) => (!( is-fireable("unlock") )) ))"# , false),  
        (r#"A(G( (tokens-count("CTL")=1) => (!( is-fireable("unlock") )) ))"# , true),  
        (r#"A(G( (tokens-count("CTL")=1) => (!( is-fireable("lock") )) ))"# , false),  
        // liveness properties
        (r#"A(G( is-fireable("lock") ))"# , false),  
        (r#"A(G( is-fireable("unlock") ))"# , false),  
        (r#"A(G( (is-fireable("lock")) | (is-fireable("unlock"))  ))"# , true),  
    ];
    for (phi_as_str,is_sat) in phis {
        let (_,phi) = ctl_parser.parse_ctl_formula::<nom::error::Error<&str>>(phi_as_str).unwrap();
        let result = is_ctl_formula_sat(&kripke_only_marks,&initial_states,&phi);
        assert_eq!(result,is_sat,"{} : \n{:?}\n", phi_as_str, phi);
    }


    let phis_with_past = vec![
        // safety properties
        (r#"A(G( (is-previous("lock")) => (A(X( !(is-previous("lock")) ))) ))"# , true),
        (r#"A(G( (is-previous("unlock")) => (A(X( !(is-previous("unlock")) ))) ))"# , true),
        (r#"A(G( (is-previous("unlock")) => (A(X( !(is-previous("lock")) ))) ))"# , false),
        (r#"A(G( (is-previous("lock")) => (A(X( !(is-previous("unlock")) ))) ))"# , false),
        // ***
        (r#"A(G( (is-previous("lock")) => (tokens-count("CTL")=0) ))"# , true),
        (r#"A(G( (is-previous("unlock")) => (tokens-count("CTL")=1) ))"# , true),
    ];
    for (phi_as_str,is_sat) in phis_with_past {
        let (_,phi) = ctl_parser.parse_ctl_formula::<nom::error::Error<&str>>(phi_as_str).unwrap();
        let result = is_ctl_formula_sat(&kripke_with_prev_labs,&initial_states,&phi);
        assert_eq!(result,is_sat,"{} : \n{:?}\n", phi_as_str, phi);
    }
}












