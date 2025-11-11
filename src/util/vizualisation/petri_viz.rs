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

use crate::{model::{marking::Marking, net::PetriNet}, util::context::PetriNetContext};





pub fn petri_repr(
    context : &PetriNetContext,
    petri : &PetriNet, 
    marking : Option<&Marking>
) -> GraphVizDiGraph {
    // Create a new graph:
    let mut digraph = GraphVizDiGraph::new(vec![]);
    // places
    for place_id in 0..petri.num_places {
        let label = if let Some(mrk) = marking {
            let num_tokens_at_place = mrk.tokens.get(place_id).unwrap();
            if *num_tokens_at_place > 0 {
                format!("tks:{}",num_tokens_at_place)
            } else {
                "".to_string()
            }
        } else{
            "".to_string()
        };
        let style = vec![
                GraphvizNodeStyleItem::Shape(GvNodeShape::Circle),
                GraphvizNodeStyleItem::Label(format!("{}\n{}",context.get_place_label(place_id),label))];
        digraph.add_node(GraphVizNode::new(format!("place{:}",place_id),style));
    }
    // transitions
    for (tr_id,transition) in petri.transitions.iter().enumerate() {
        let label = context.get_transition_label_from_transition_id(tr_id);
        let style = vec![
                GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                GraphvizNodeStyleItem::Label(label.to_string())];
        digraph.add_node(GraphVizNode::new(format!("tr{:}",tr_id),style));
        for (preset_place,preset_req_num_toks) in transition.preset_tokens.tokens.iter().enumerate() {
            if *preset_req_num_toks > 0 {
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
        }
        for (postset_place,postset_added_toks) in transition.postset_tokens.tokens.iter().enumerate() {
            if *postset_added_toks > 0 {
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
    }
    digraph
}