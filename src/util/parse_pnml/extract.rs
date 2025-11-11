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

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::util::parse_pnml::error::PnmlParsingError;
use crate::util::parse_pnml::syntax::*;


pub struct PnmlParsingFirstPass {
    pub number_of_places : usize,
    pub place_text_id_to_int_id : HashMap<String,usize>,
    pub initial_marking : Vec<u32>,
    pub transitions_text_ids : HashSet<String>,
    pub raw_arcs : Vec<(String,String)>
}

impl PnmlParsingFirstPass {
    fn new(number_of_places: usize, place_text_id_to_int_id: HashMap<String,usize>, initial_marking: Vec<u32>, transitions_text_ids: HashSet<String>, raw_arcs: Vec<(String,String)>) -> Self {
        Self { number_of_places, place_text_id_to_int_id, initial_marking, transitions_text_ids, raw_arcs }
    }
}


pub fn read_pnml_first_pass<R: BufRead>(mut reader: EventReader<R>) -> Result<PnmlParsingFirstPass, PnmlParsingError> {
    let mut next_place = 0;
    let mut place_text_id_to_int_id : HashMap<String,usize> = HashMap::new();
    let mut initial_marking : Vec<u32> = Vec::new();
    let mut transitions_text_ids : HashSet<String> = HashSet::new();
    let mut raw_arcs : Vec<(String,String)> = Vec::new();
    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement{name,attributes,..}) => match name.local_name.as_str() {
                PNML_PLACE => {
                    let attrs = collect_attributes(attributes);
                    let (id,opt_init_mark) = read_place(&mut reader, attrs)?;
                    let place_int_id = next_place;
                    next_place +=1;
                    place_text_id_to_int_id.insert(id, place_int_id);
                    if let Some(init_num_tokens_at_place) = opt_init_mark {
                        initial_marking.push(init_num_tokens_at_place);
                    } else {
                        initial_marking.push(0);
                    }
                },
                PNML_TRANSITION => {
                    let attrs = collect_attributes(attributes);
                    let id = read_transition(&mut reader, attrs)?;
                    transitions_text_ids.insert(id);
                }
                PNML_ARC => {
                    let attrs = collect_attributes(attributes);
                    let (source,target) = read_arc(&mut reader, attrs)?;
                    raw_arcs.push((source,target));
                }
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name.as_str() == PNML_NET {
                    break;
                }
            },
            Err(e) => {
                return Err(PnmlParsingError::Xml(e))
            }
            _ => {}
        }
    }
    Ok(PnmlParsingFirstPass::new(next_place, place_text_id_to_int_id, initial_marking, transitions_text_ids, raw_arcs))
}






fn read_place<R: BufRead>(
    reader: &mut EventReader<R>,
    mut attrs : HashMap<String,String>
) -> Result<(String,Option<u32>),PnmlParsingError> {
    let id: String = attrs.remove(PNML_TEXT_ID).ok_or(PnmlParsingError::MissingAttribute{att:PNML_TEXT_ID,parent:PNML_PLACE})?;
    let mut opt_init_mark = None;
    loop {
        match reader.next() {
            Err(e) => {return Err(PnmlParsingError::Xml(e))}
            Ok(XmlEvent::StartElement{name,..}) => if name.local_name.as_str() == PNML_INITIAL_MARKING {
                let num_toks = read_initial_marking(reader)?;
                opt_init_mark = Some(num_toks);
            }
            Ok(XmlEvent::EndElement{name}) => {
                if name.local_name == PNML_PLACE {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok((id,opt_init_mark))
}


fn read_initial_marking<R: BufRead>(
    reader: &mut EventReader<R>
) -> Result<u32,PnmlParsingError> {
    let mut num_tokens = None;
    loop {
        match reader.next() {
            Err(e) => {return Err(PnmlParsingError::Xml(e))}
            Ok(XmlEvent::StartElement{name,..}) => if name.local_name.as_str() == PNML_TEXT {
                let txt = read_text_then_close(reader,PNML_TEXT)?;
                match txt.parse::<u32>() {
                    Ok(num_toks) => {
                        num_tokens = Some(num_toks);
                    }
                    Err(_) => {
                        return Err(PnmlParsingError::CouldNotParseInitialMarkingTokenNumberToInteger);
                    }
                }
            }
            Ok(XmlEvent::EndElement{name}) => {
                if name.local_name == PNML_INITIAL_MARKING {
                    break;
                }
            }
            _ => {}
        }
    }
    match num_tokens {
        None => {
            Err(PnmlParsingError::MissingNumberOfTokensInInitialMarking)
        },
        Some(num_toks) => {
            Ok(num_toks)
        }
    }
}

fn read_transition<R: BufRead>(
    reader: &mut EventReader<R>,
    mut attrs : HashMap<String,String>
) -> Result<String,PnmlParsingError> {
    let id: String = attrs.remove(PNML_TEXT_ID).ok_or(PnmlParsingError::MissingAttribute{att:PNML_TEXT_ID,parent:PNML_TRANSITION})?;
    loop {
        match reader.next() {
            Err(e) => {return Err(PnmlParsingError::Xml(e))}
            Ok(XmlEvent::EndElement{name}) => {
                if name.local_name == PNML_TRANSITION {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(id)
}


fn read_arc<R: BufRead>(
    reader: &mut EventReader<R>,
    mut attrs : HashMap<String,String>
) -> Result<(String,String),PnmlParsingError> {
    let source: String = attrs.remove(PNML_SOURCE).ok_or(PnmlParsingError::MissingAttribute{att:PNML_SOURCE,parent:PNML_ARC})?;
    let target: String = attrs.remove(PNML_TARGET).ok_or(PnmlParsingError::MissingAttribute{att:PNML_TARGET,parent:PNML_ARC})?;
    loop {
        match reader.next() {
            Err(e) => {return Err(PnmlParsingError::Xml(e))}
            Ok(XmlEvent::EndElement{name}) => {
                if name.local_name == PNML_ARC {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok((source,target))
}





fn collect_attributes(attributes : Vec<OwnedAttribute>) -> HashMap<String, String> {
    attributes.into_iter()
        .map(|attribute| {
            (attribute.name.local_name,attribute.value)
        })
        .collect::<HashMap<String, String>>()
}


fn read_text_then_close<R: BufRead>(
    reader: &mut EventReader<R>,
    expected_end_tag : &'static str
) -> Result<String,PnmlParsingError> {
    let got_text = match reader.next() {
        Err(e) => Err(PnmlParsingError::Xml(e)),
        Ok(XmlEvent::Characters(txt)) => {
            Ok(txt)
        },
        _ => {
            Err(PnmlParsingError::ExpectedTextStart{tag:expected_end_tag})
        }
    }?;
    match reader.next() {
        Err(e) => Err(PnmlParsingError::Xml(e)),
        Ok(XmlEvent::EndElement{name}) => {
            if name.local_name == expected_end_tag {
                Ok(got_text)
            } else {
                Err(PnmlParsingError::ExpectedTextEnd{tag:expected_end_tag})
            }
        },
        _ => {
            Err(PnmlParsingError::ExpectedTextEnd{tag:expected_end_tag})
        }
    }
}
