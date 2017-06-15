use collections::string::String;
use collections::vec::Vec;
use collections::boxed::Box;

use core::str::FromStr;

use super::namedobj::RegionSpace;

#[derive(Debug)]
pub struct AmlNamespace {
    name: String,
    contents: AmlNamespaceContents
}

#[derive(Debug)]
pub enum AmlNamespaceContents {
    Value(AmlValue),
    SubNamespace(Box<AmlNamespace>),
    Namespace(Vec<AmlNamespaceContents>),
    OpRegion {
        region: RegionSpace,
        offset: AmlValue,
        len: AmlValue
    }
}

#[derive(Debug)]
pub enum AmlValue {
    Uninitialized,
    Buffer,
    BufferField,
    DDBHandle,
    DebugObject,
    Device,
    Event,
    FieldUnit,
    Integer,
    IntegerConstant,
    Method,
    Mutex,
    ObjectReference,
    OperationRegion,
    Package,
    String,
    PowerResource,
    Processor,
    RawDataBuffer,
    ThermalZone
}

impl AmlNamespace {
    pub fn new_namespace(name: &String) -> AmlNamespace {
        AmlNamespace {
            name: name.clone(),
            contents: AmlNamespaceContents::Namespace(vec!())
        }
    }
    
    pub fn push(&mut self, val: AmlNamespaceContents) {
        match self.contents {
            AmlNamespaceContents::Namespace(ref mut v) => v.push(val),
            _ => () // TODO: Error this
        }
    }

    pub fn push_to(&mut self, scope_string: String, contents: AmlNamespaceContents) {
        if scope_string.len() == 0 {
            return;
        }
        
        let mut scope_string = scope_string.clone();
        
        if scope_string.starts_with("\\") {
            if self.name != "\\" {
                return;
                // TODO: Error this
            }

            scope_string.remove(0);
        }

        if scope_string.starts_with(".") {
            scope_string.remove(0);
        }
        
        if scope_string.len() == 0 {
            return;
        }

        let (current, nextset) = match scope_string.find(".") {
            Some(s) => {
                let (x, mut y) = scope_string.split_at(s);
                y = &y[1..];

                (String::from_str(x).unwrap(), String::from_str(y).unwrap())
            },
            None => if scope_string.len() <= 4 {
                (scope_string, String::from_str("").unwrap())
            } else {
                return;
            }
        };

        match self.contents {
            AmlNamespaceContents::Namespace(ref mut namespace) => {
                // TODO: Remove this while loop here, there has to be a more elegant way
                let mut current_index = 0;
                while current_index < namespace.len() {
                    match namespace[current_index] {
                        AmlNamespaceContents::SubNamespace(ref mut ns) => if ns.name == current {
                            ns.push_to(nextset, contents);
                            return;
                        },
                        _ => ()
                    }

                    current_index += 1;
                }
                
                let mut next = AmlNamespace {
                    name: current,
                    contents: contents
                };
                
                namespace.push(AmlNamespaceContents::SubNamespace(Box::new(next)));
            }
            _ => () // TODO: Error this
        }
    }

    pub fn push_subordinate_namespace(&mut self, scope_string: String) {
        self.push_to(scope_string, AmlNamespaceContents::Namespace(vec!()));
    }
}