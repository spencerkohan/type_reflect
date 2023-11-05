use std::{ffi::OsStr, path::Path};

use dprint_plugin_typescript::configuration::{
    ConfigurationBuilder, NextControlFlowPosition, QuoteStyle,
};

use crate::{AliasType, EnumReflectionType, StructType, TypeEmitter};

pub struct TSFormat {
    pub tab_size: u8,
    pub line_width: u32,
}

impl Default for TSFormat {
    fn default() -> Self {
        Self {
            tab_size: 2,
            line_width: 80,
        }
    }
}

impl TypeEmitter for TSFormat {
    fn prefix(&mut self) -> String {
        "".to_string()
    }

    fn emit_struct<T>(&mut self) -> String
    where
        T: StructType,
    {
        "".to_string()
    }

    fn emit_enum<T>(&mut self) -> String
    where
        T: EnumReflectionType,
    {
        "".to_string()
    }

    fn emit_alias<T>(&mut self) -> String
    where
        T: AliasType,
    {
        "".to_string()
    }

    fn finalize<P>(&mut self, path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        // build the configuration once
        let config = ConfigurationBuilder::new()
            .indent_width(self.tab_size)
            .line_width(self.line_width)
            .prefer_hanging(true)
            .prefer_single_line(false)
            .quote_style(QuoteStyle::PreferSingle)
            .next_control_flow_position(NextControlFlowPosition::SameLine)
            .build();

        let file_path = Path::new(&path);

        let text: String = std::fs::read_to_string(Path::new(&path))?;

        let result =
            dprint_plugin_typescript::format_text(Path::new(&path), text.as_str(), &config);

        match result {
            Ok(Some(contents)) => {
                std::fs::write(file_path, contents)?;
            }
            Err(_) => {}
            _ => {}
        };

        Ok(())
    }
}
