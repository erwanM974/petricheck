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

use std::{collections::{HashMap}, fs::File, io::{BufRead, BufReader}};

use xml::{reader::XmlEvent, EventReader};

use crate::{model::{marking::Marking, net::PetriNet, transition::PetriTransition}, util::parse_pnml::{error::PnmlParsingError, extract::read_pnml_first_pass}};

use crate::util::parse_pnml::syntax::*;



pub struct PnmlFileContent {
    pub petri_net : PetriNet,
    pub initial_marking : Marking,
    pub place_text_id_to_int_id : HashMap<String,usize>,
}

impl PnmlFileContent {
    pub fn new(petri_net: PetriNet, initial_marking: Marking, place_text_id_to_int_id: HashMap<String,usize>) -> Self {
        Self { petri_net, initial_marking, place_text_id_to_int_id }
    }
}


pub fn read_petri_net_from_pnml_file_path(path : &str) -> Result<PnmlFileContent,PnmlParsingError> {
    match File::open(path) {
        Ok(file) => {
            let file = BufReader::new(file);
            let reader = EventReader::new(file);
            read_pnml(reader)
        },
        Err(_) => {
            Err(PnmlParsingError::CouldNotOpenFile)
        },
    }
}




// Read PNML content and return the Petri Net
pub fn read_pnml<R: BufRead>(mut reader: EventReader<R>) -> Result<PnmlFileContent, PnmlParsingError> {
    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement{name,attributes,namespace}) => {
                if name.local_name.as_str() == PNML_NET {
                    return read_pnml_content(reader);
                } else {
                    println!("got {:?}", XmlEvent::StartElement{name,attributes,namespace});
                }
            },
            Ok(x) => {
                println!("got {:?}", x);
            },
            Err(e) => {
                return Err(PnmlParsingError::Xml(e))
            }
        }
    }
}


// Read PNML content and return the Petri Net
pub fn read_pnml_content<R: BufRead>(reader: EventReader<R>) -> Result<PnmlFileContent, PnmlParsingError> {
    let first_pass_result = read_pnml_first_pass(reader)?;
    // ***
    let mut transitions = Vec::new();
    {
        let mut transitions_incoming = HashMap::new();
        let mut transitions_outgoing = HashMap::new();
        for tr in &first_pass_result.transitions_text_ids {
            transitions_incoming.insert(tr.clone(), Marking::new_empty(first_pass_result.number_of_places));
            transitions_outgoing.insert(tr.clone(), Marking::new_empty(first_pass_result.number_of_places));
        }
        for (source_id,target_id) in first_pass_result.raw_arcs {
            if let Some(source_place_id) = first_pass_result.place_text_id_to_int_id.get(&source_id) {
                let tx_incoming_places = transitions_incoming.get_mut(&target_id).ok_or(PnmlParsingError::UnknownTransition)?;
                *tx_incoming_places.tokens.get_mut(*source_place_id).unwrap() += 1;
            } else if let Some(target_place_id) = first_pass_result.place_text_id_to_int_id.get(&target_id) {
                let tx_outgoing_places = transitions_outgoing.get_mut(&source_id).ok_or(PnmlParsingError::UnknownTransition)?;
                *tx_outgoing_places.tokens.get_mut(*target_place_id).unwrap() += 1;
            } else {
                return Err(PnmlParsingError::NeitherSourceNotTargetOfArcIsAPlace)
            }
        }
        for transition in first_pass_result.transitions_text_ids {
            let from_places = transitions_incoming.remove(&transition).unwrap();
            let to_places = transitions_outgoing.remove(&transition).unwrap();
            transitions.push(PetriTransition::new(from_places, to_places));
        }
    }
    let net = PetriNet::new(first_pass_result.number_of_places, transitions);
    let marking = Marking::new(first_pass_result.initial_marking);
    Ok(PnmlFileContent::new(net,marking,first_pass_result.place_text_id_to_int_id))
}



