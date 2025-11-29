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




use graphviz_dot_builder::{edge::{edge::GraphVizEdge, style::GraphvizEdgeStyleItem}, graph::graph::GraphVizDiGraph, item::node::{node::GraphVizNode, style::{GraphvizNodeStyleItem, GvNodeShape}}, traits::DotBuildable};

use crate::model::{marking::Marking, net::PetriNet};



pub fn petri_repr(
    petri : &PetriNet, 
    marking : Option<&Marking>
) -> GraphVizDiGraph {
    // Create a new graph:
    let mut digraph = GraphVizDiGraph::new(vec![]);
    // places
    for (place_id,place_content) in petri.places.iter().enumerate() {
        let mut label = format!("p{:}",place_id);
        if let Some(lab_ref) = place_content {
            label.push_str(&format!(":({:})", lab_ref));
        };
        if let Some(mrk) = marking {
            if let Some(num_tokens_at_place) = mrk.get_num_toks_at_place(&place_id) {
                debug_assert!(*num_tokens_at_place >0);
                label.push_str(&format!("\ntks:{:}", num_tokens_at_place));
            }
        };
        let style = vec![
                GraphvizNodeStyleItem::Shape(GvNodeShape::Circle),
                GraphvizNodeStyleItem::Label(label)];
        digraph.add_node(GraphVizNode::new(format!("place{:}",place_id),style));
    }
    // transitions
    for (tr_id,transition) in petri.transitions.iter().enumerate() {
        let transition_label = match &transition.transition_label {
            Some(tr_lab) => {
                format!("{:}",tr_lab)
            },
            None => {
                "".to_string()
            }
        };
        let style = vec![
                GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                GraphvizNodeStyleItem::Label(transition_label)];
        digraph.add_node(GraphVizNode::new(format!("tr{:}",tr_id),style));
        for (preset_place,preset_req_num_toks) in transition.iter_preset_tokens() {
            debug_assert!(*preset_req_num_toks>0);
            let style = if *preset_req_num_toks > 1 {
                vec![GraphvizEdgeStyleItem::Label(preset_req_num_toks.to_string())]
            } else {
                Vec::new()
            };
            let edge = GraphVizEdge::new(
                format!("place{:}",preset_place),
                None,
                format!("tr{:}",tr_id),
                None,
                style
            );
            digraph.add_edge(edge);
        }
        for (postset_place,postset_added_toks) in transition.iter_postset_tokens() {
            debug_assert!(*postset_added_toks>0);
            let style = if *postset_added_toks > 1 {
                vec![GraphvizEdgeStyleItem::Label(postset_added_toks.to_string())]
            } else {
                Vec::new()
            };
            let edge = GraphVizEdge::new(
                format!("tr{:}",tr_id),
                None,
                format!("place{:}",postset_place),
                None,
                style
            );
            digraph.add_edge(edge);
        }
    }
    digraph
}



