use std::borrow::Borrow;

pub trait DataTypeMapper {
    fn map_data_type(self) -> String;
}

impl DataTypeMapper for &str {
    fn map_data_type(self) -> String {
        let (has_key, default_key_type) = (true, "UNIQUEIDENTIFIER");
        let data_type = self.to_lowercase().replace('"', "");
        let result = match data_type.as_str() {
            "string" => "VARCHAR(MAX)",
            "integer" | "int" => "INTEGER",
            "bool" | "boolean" => "BIT",
            "date" => "DATE",
            "text" => "TEXT",
            _ if !has_key => "",
            _ if has_key => default_key_type,
            _ => ""
        };
        return result.to_owned();
    }
}