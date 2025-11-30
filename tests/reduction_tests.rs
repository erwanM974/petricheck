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

use petricheck::{model::{label::{PetriTransitionLabel}, marking::Marking, net::PetriNet, transition::PetriTransition}, reduction::reduce::reduce_petri_net, util::{vizualisation::petri_viz::petri_repr}};
use graphviz_dot_builder::traits::{DotPrintable, GraphVizOutputFormat};
use map_macro::{btree_map, hash_map};




#[test]
pub fn test_simple_example() {
    let tr_a = Rc::new(PetriTransitionLabel::new("A".to_string()));
    let tr_b = Rc::new(PetriTransitionLabel::new("B".to_string()));
    let tr_c = Rc::new(PetriTransitionLabel::new("C".to_string()));
    let mut petri_net = PetriNet::new(
        vec![
            None,None,None,None
        ], 
        vec![
            PetriTransition::new(
                Some(tr_a.clone()),
                hash_map! {0=>1},
                hash_map! {1=>1}
            ),
            PetriTransition::new(
                Some(tr_b.clone()),
                hash_map! {1=>1},
                hash_map! {2=>1}
            ),
            PetriTransition::new(
                Some(tr_c.clone()),
                hash_map! {2=>1},
                hash_map! {3=>1}
            )
        ]
    );
    let mut initial_marking = Some(Marking::new(btree_map! {0=>1}));
    {
        let gv = petri_repr(&petri_net,&initial_marking);
        gv.print_dot(&[".".to_string()], "initial", &GraphVizOutputFormat::png).unwrap();
    }
    // ***
    petri_net.relabel_transitions(hash_map! {(*tr_b).clone()=>None});
    {
        let gv = petri_repr(&petri_net,&initial_marking);
        gv.print_dot(&[".".to_string()], "after_relabel", &GraphVizOutputFormat::png).unwrap();
    }
    // ***
    reduce_petri_net(&mut petri_net, &mut initial_marking);
    {
        let gv = petri_repr(&petri_net,&initial_marking);
        gv.print_dot(&[".".to_string()], "after_reduction", &GraphVizOutputFormat::png).unwrap();
    }
}












