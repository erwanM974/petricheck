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

use petricheck::{model::{label::{PetriTransitionLabel}, marking::Marking, net::PetriNet, transition::PetriTransition}, reduction::reduce::reduce_petri_net, util::{vizualisation::petri_viz::petri_repr}};
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::{btree_map, hash_map};





fn tool_test_pn_reduction(
        title : &str, 
        pn : PetriNet, 
        im : Option<Marking>, 
        relabbelling : HashMap<PetriTransitionLabel, Option<Rc<PetriTransitionLabel>>>,
        expected_reduced_pn : PetriNet,
        expected_reduced_im : Option<Marking>
    ) {
    {
        let gv = petri_repr(&pn,&im);
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}_1initial", title), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    let mut transformed_pn = pn.clone();
    let mut transformed_im = im.clone();
    // assert reduction on fully labelled pn does not change it
    reduce_petri_net(&mut transformed_pn, &mut transformed_im);
    assert_eq!(transformed_pn,pn);
    assert_eq!(transformed_im,im);
    // relabel transitions
    transformed_pn.relabel_transitions(relabbelling);
    {
        let gv = petri_repr(&transformed_pn,&transformed_im);
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}_2relabelled", title), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    // apply reduction
    reduce_petri_net(&mut transformed_pn, &mut transformed_im);
    {
        let gv = petri_repr(&transformed_pn,&transformed_im);
        gv.print_dot(
            &[".".to_string()], 
            &format!("{}_3reduced", title), 
            &GraphVizOutputFormat::png
        ).unwrap();
    }
    // check obtained is expected
    assert_eq!(transformed_pn,expected_reduced_pn);
    assert_eq!(transformed_im,expected_reduced_im);
}


#[test]
pub fn test_series_places1() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let tr_b = Rc::new(PetriTransitionLabel::new("B".to_string()));
    let pn = PetriNet::new(
        vec![
            None,None,None
        ], 
        vec![
            PetriTransition::new(
                Some(tr_a.clone()),
                hash_map! {0=>1},
                hash_map! {2=>1}
            ),
            PetriTransition::new(
                Some(tr_b.clone()),
                hash_map! {1=>1},
                hash_map! {2=>1}
            )
        ]
    );
    let im = Some(Marking::new(btree_map! {0=>1}));
    let relabelling = hash_map! {(*tr_a).clone()=>None};
    let expected_reduced_pn = PetriNet::new(
        vec![
            None,None
        ], 
        vec![
            PetriTransition::new(
                Some(tr_b.clone()),
                hash_map! {0=>1},
                hash_map! {1=>1}
            )
        ]
    );
    let expected_reduced_im = Some(Marking::new(btree_map! {1=>1}));
    tool_test_pn_reduction(
        "series_places1",
        pn,
        im,
        relabelling,
        expected_reduced_pn,
        expected_reduced_im
    );
}






#[test]
pub fn test_series_transitions1() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let tr_b = Rc::new(PetriTransitionLabel::new("B".to_string()));
    let tr_c = Rc::new(PetriTransitionLabel::new("C".to_string()));
    let pn = PetriNet::new(
        vec![
            None,None,None
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
                hash_map! {2=>1}
            ),
            PetriTransition::new(
                Some(tr_c.clone()),
                hash_map! {2=>1},
                hash_map! {0=>1}
            )
        ]
    );
    let im = Some(Marking::new(btree_map! {0=>1}));
    let relabelling = hash_map! {(*tr_c).clone()=>None};
    let expected_reduced_pn = PetriNet::new(
        vec![
            None,None
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
                hash_map! {0=>1}
            )
        ]
    );
    let expected_reduced_im = im.clone();
    tool_test_pn_reduction(
        "series_transitions1",
        pn,
        im,
        relabelling,
        expected_reduced_pn,
        expected_reduced_im
    );
}








#[test]
pub fn test_series_transitions2() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let tr_b = Rc::new(PetriTransitionLabel::new("B".to_string()));
    let tr_c = Rc::new(PetriTransitionLabel::new("C".to_string()));
    let pn = PetriNet::new(
        vec![
            None,None,None
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
                hash_map! {2=>1}
            ),
            PetriTransition::new(
                Some(tr_c.clone()),
                hash_map! {2=>1},
                hash_map! {0=>1}
            )
        ]
    );
    let im = Some(Marking::new(btree_map! {0=>1}));
    let relabelling = hash_map! {(*tr_b).clone()=>None};
    let expected_reduced_pn = PetriNet::new(
        vec![
            None,None
        ], 
        vec![
            PetriTransition::new(
                Some(tr_a.clone()),
                hash_map! {0=>1},
                hash_map! {1=>1}
            ),
            PetriTransition::new(
                Some(tr_c.clone()),
                hash_map! {0=>1},
                hash_map! {0=>1}
            )
        ]
    );
    let expected_reduced_im = im.clone();
    tool_test_pn_reduction(
        "series_transitions2",
        pn,
        im,
        relabelling,
        expected_reduced_pn,
        expected_reduced_im
    );
}








