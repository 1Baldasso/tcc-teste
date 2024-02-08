use json::JsonValue;
use arrow::util::string_writer::StringWriter;
use std::io::Write;
use crate::{transpiler::type_mapper::DataTypeMapper, Cli};
use std::borrow::Borrow;

pub trait NodeVisitor {
    fn visit_node(&mut self, writer: &mut StringWriter, args: &Cli) -> String;
}

impl NodeVisitor for JsonValue {
    fn visit_node(&mut self,  writer: &mut StringWriter, args: &Cli) -> String {
        let json_val = self;
        if json_val.is_string() {
            return json_val.dump().map_data_type(!args.no_keys, args.key_type.borrow());
        } else if json_val.is_object() {
            for (key, val) in json_val.entries_mut() {
                if val.is_object()
                {
                    let table_string = format!("CREATE TABLE {} (\n", key);
                    let _ = writer.write(table_string.as_bytes());
                }
                let result = val.visit_node(writer, args);
                if !result.is_empty()
                {
                    let formatted_string: String;
                    if result.to_lowercase() == args.key_type.to_lowercase() {
                        formatted_string = format!("{}Id {} FOREIGN KEY REFERENCES {}(Id)", key, result, key)
                    }
                    else {

                        formatted_string = format!("{} {},\n", key, result);
                    }

                    let bytes = formatted_string.as_bytes();
                    let _ = writer.write(bytes);
                } else {
                    let _ = writer.write(String::from(")\n").as_bytes());
                }
            }
            
            return String::from("");
        }
        return String::from("");
    }
}

