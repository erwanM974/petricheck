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

use std::{collections::{HashMap, HashSet}, rc::Rc};

use petricheck::{model::{label::{PetriStateLabel, PetriTransitionLabel}, marking::Marking, net::PetriNet, transition::PetriTransition}, model_checking::to_kripke::{PetriKripkeGenerationSafenessRequirement, PetriKripkeStateProducer, petri_to_kripke}, util::{parse_ctl::parser::BuiltinPetriCtlParser, vizualisation::{kripke_viz::PetriKripkeVisualizer, petri_viz::petri_repr}}};
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::{btree_map, hash_map, hash_set};

use citreelo::util::viz_kripke::KripkeStructureGraphvizDrawer;

use citreelo::solve::is_ctl_formula_sat;
use citreelo::parser::CtlFormulaParser;






fn tool_test_pn_mc(
    title : &str, 
    pn : PetriNet, 
    im : Marking, 
    initial_states: HashSet<usize>,
    tags_and_formulaes : Vec<(&'static str,HashSet<PetriTransitionLabel>,HashMap<&'static str, bool>)>
    /*formulae_no_prev_labs : HashMap<&'static str, bool>,
    tagged_prev_transition_labels : HashSet<PetriTransitionLabel>,
    formulae_with_prev_labs : HashMap<&'static str, bool>,*/
) {
    let folder_name = "test_modcheck";
    let _ = std::fs::create_dir(folder_name);
    {
        let gv = petri_repr(&pn,&Some(im.clone()));
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}/{}_1initial", folder_name, title), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    let ctl_parser = BuiltinPetriCtlParser::from_net(&pn).unwrap();
    for (index,(tags_title,tagged_prev_labels, formulaes)) in tags_and_formulaes
        .into_iter()
        .enumerate() 
    {
        let kripke = petri_to_kripke(
            &pn,
            im.clone(),
            &PetriKripkeStateProducer::new(tagged_prev_labels),
            &PetriKripkeGenerationSafenessRequirement::KSafeness(1)
        ).unwrap();
        {
            let gv = PetriKripkeVisualizer::new(&pn).get_kripke_repr(&kripke);
            gv.print_dot(
                &[".".to_string()], 
                &format!("{}/{}_{}{}_kripke", folder_name, title, index, tags_title), 
                &GraphVizOutputFormat::png
            ).unwrap();
        }

        for (phi_as_str,is_sat) in formulaes {
            let (_,phi) = ctl_parser.parse_ctl_formula::<nom::error::Error<&str>>(
                phi_as_str
            ).unwrap();
            let result = is_ctl_formula_sat(
                &kripke,
                &initial_states,
                &phi
            );
            assert_eq!(result,is_sat,"{:} ({}) : {} -> {} | expected {}", title, tags_title, phi_as_str, result, is_sat);
        }
    }
}


#[test]
pub fn test_lock_unlock() {
    let lock_tr = Rc::new(PetriTransitionLabel::new("lock".to_string()));
    let unlock_tr = Rc::new(PetriTransitionLabel::new("unlock".to_string()));
    let pn = PetriNet::new(
        vec![
            Some(Rc::new(PetriStateLabel::new("A_U".to_string()))),
            Some(Rc::new(PetriStateLabel::new("A_L".to_string()))),
            Some(Rc::new(PetriStateLabel::new("CTL".to_string()))),
            Some(Rc::new(PetriStateLabel::new("B_U".to_string()))),
            Some(Rc::new(PetriStateLabel::new("B_L".to_string()))),
        ], 
        vec![
            PetriTransition::new(
                Some(lock_tr.clone()),
                hash_map! {0=>1,2=>1},
                hash_map! {1=>1}
            ),
            PetriTransition::new(
                Some(lock_tr.clone()),
                hash_map! {3=>1,2=>1},
                hash_map! {4=>1}
            ),
            PetriTransition::new(
                Some(unlock_tr.clone()),
                hash_map! {1=>1},
                hash_map! {0=>1,2=>1}
            ),
            PetriTransition::new(
                Some(unlock_tr.clone()),
                hash_map! {4=>1},
                hash_map! {3=>1,2=>1}
            )
        ]
    );
    let im = Marking::new(btree_map! {0=>1,2=>1,3=>1});
    let initial_states = hash_set! {0};
    
    let formulae_no_prev_labs = hash_map!{
        // at initial state
        r#"tokens-count("A_L")=0"# => true,  
        r#"tokens-count("A_L")>0"# => false,  
        r#"tokens-count("B_L")=0"# => true ,  
        r#"tokens-count("B_L")>0"# => false,  
        r#"tokens-count("A_U")=1"# => true ,  
        r#"tokens-count("B_U")=1"# => true ,  
        r#"tokens-count("A_U")=0"# => false,  
        r#"tokens-count("B_U")=0"# => false,  
        r#"tokens-count("CTL")=1"# => true ,  
        r#"tokens-count("CTL")=0"# => false,  
        // safety properties
        r#"A(G(!((tokens-count("A_U")>0)&(tokens-count("B_U")>0))))"#                                                      => false,  
        r#"A(G(!((tokens-count("A_L")>0)&(tokens-count("B_L")>0))))"#                                                      => true ,  
        r#"A(G( ((tokens-count("A_L")=0)&(tokens-count("A_U")=1)) | ((tokens-count("A_L")=1)&(tokens-count("A_U")=0)) ))"# => true ,  
        r#"A(G( ((tokens-count("B_L")=0)&(tokens-count("B_U")=1)) | ((tokens-count("B_L")=1)&(tokens-count("B_U")=0)) ))"# => true ,  
        r#"A(G( (tokens-count("CTL")=0) => (!( is-fireable("lock") )) ))"#                                                 => true ,  
        r#"A(G( (tokens-count("CTL")=0) => (!( is-fireable("unlock") )) ))"#                                               => false,  
        r#"A(G( (tokens-count("CTL")=1) => (!( is-fireable("unlock") )) ))"#                                               => true ,  
        r#"A(G( (tokens-count("CTL")=1) => (!( is-fireable("lock") )) ))"#                                                 => false,  
        // liveness properties
        r#"A(G( is-fireable("lock") ))"#                              => false,  
        r#"A(G( is-fireable("unlock") ))"#                            => false,  
        r#"A(G( (is-fireable("lock")) | (is-fireable("unlock"))  ))"# => true
    };
    
    let formulae_with_prev_labs = hash_map!{
        // safety properties
        r#"A(G( (is-previous("lock")) => (A(X( !(is-previous("lock")) ))) ))"#     => true ,
        r#"A(G( (is-previous("unlock")) => (A(X( !(is-previous("unlock")) ))) ))"# => true ,
        r#"A(G( (is-previous("unlock")) => (A(X( !(is-previous("lock")) ))) ))"#   => false,
        r#"A(G( (is-previous("lock")) => (A(X( !(is-previous("unlock")) ))) ))"#   => false,
        // ***
        r#"A(G( (is-previous("lock")) => (tokens-count("CTL")=0) ))"#   => true,
        r#"A(G( (is-previous("unlock")) => (tokens-count("CTL")=1) ))"# => true,
    };

    tool_test_pn_mc(
        "lock_unlock",
        pn,
        im,
        initial_states,
        vec![
            ("no_prev_labs",hash_set!{},formulae_no_prev_labs.clone()),
            ("base_formulae_kripke_with_prev_labs",hash_set! {(*lock_tr).clone(),(*unlock_tr).clone()},formulae_no_prev_labs),
            ("formulae_with_prev_labs_kripke_with_prev_labs",hash_set! {(*lock_tr).clone(),(*unlock_tr).clone()},formulae_with_prev_labs),
        ]
    )
}








