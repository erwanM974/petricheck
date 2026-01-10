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



use std::rc::Rc;

use petricheck::{model::{label::PetriTransitionLabel, marking::Marking, net::PetriNet, transition::PetriTransition}, model_checking::to_kripke::{PetriKripkeGenerationError, PetriKripkeGenerationSafenessRequirement, PetriKripkeStateProducer}, util::vizualisation::petri_viz::petri_repr};
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::{btree_map, hash_map, hash_set};

use petricheck::model_checking::to_kripke::petri_to_kripke;



fn tool_test_ksafeness_violation_detection(
        title : &str, 
        pn : PetriNet, 
        im : Marking, 
        k_val : u32,
        // if violation gives place_id, transition_id
        violation : Option<(usize,usize)>
    ) {
    {
        let gv = petri_repr(&pn,&Some(im.clone()));
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}_test_ksafe_k{}", title, k_val), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    let got_result = petri_to_kripke(
        &pn,
        im,
        &PetriKripkeStateProducer::new(hash_set! {}),
        &PetriKripkeGenerationSafenessRequirement::KSafeness(k_val)
    );
    match violation {
        None => {
            assert!(got_result.is_ok());
        },
        Some((violation_place_id, violation_transition_id)) => {
            if let Err(gen_err) = got_result {
                match gen_err {
                    PetriKripkeGenerationError::KSafenessViolation { place_id, transition_id } => {
                        assert_eq!(place_id,violation_place_id);
                        assert_eq!(transition_id,violation_transition_id);
                    }
                }
            } else {    
                panic!("should have detected k-safeness violation");
            }
        }
    }
}


#[test]
pub fn test_not_1safe1() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let pn = PetriNet::new(
        vec![
            None,None
        ], 
        vec![
            PetriTransition::new(
                Some(tr_a.clone()),
                hash_map! {0=>1},
                hash_map! {0=>1,1=>1}
            )
        ]
    );
    let im = Marking::new(btree_map! {0=>1});
    tool_test_ksafeness_violation_detection(
        "not_1safe1",pn,im,1,Some((1,0))
    )
}
















