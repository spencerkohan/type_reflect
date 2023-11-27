use deno_ast::{parse_module, Diagnostic, SourceTextInfo};
use dprint_plugin_typescript::{
    configuration::{Configuration, ConfigurationBuilder, NextControlFlowPosition, QuoteStyle},
    format_parsed_source,
};
pub use ts_quote_macros::ts_quote;
pub use ts_quote_macros::ts_string;

pub use deno_ast::ParsedSource as TS;

/**
The TSSource trait is used to add a few convenience methods to the  deno_ast::ParsedSource type.
**/
pub trait TSSource: Sized {
    /**
    Creates a ParsedSource instance from a string.

    # Arguments:

    * `source` - A TypeScript source string

    # Returns

    Returns a ParsedSource, or an error diagnostic if source is not valid TypeScript
    **/
    fn from_source(source: String) -> Result<Self, Diagnostic>;

    /**
    Returns a formatted TypeScript string.

    # Arguments:

    * `config` - Optional: a `dprint_plugin_typescript` config used for formatting the output.

    If no config is provided, the function will output using the default config:
    - `line_width`: `80`
    - `indent_width`: `2`
    - `prefer_hanging`: `true`
    - `prefer_single_line`: `false`
    - `quote_style`: `QuoteStyle::PreferSingle`
    - `next_control_flow_position`: `NextControlFlowPosition::SameLine`

    # Returns

    Returns a ParsedSource, or an error diagnostic if source is not valid TypeScript
    **/
    fn formatted(&self, config: Option<&Configuration>) -> anyhow::Result<String>;
}

impl TSSource for TS {
    fn from_source(source: String) -> Result<Self, Diagnostic> {
        parse_module(deno_ast::ParseParams {
            specifier: "".to_string(),
            text_info: SourceTextInfo::from_string(source),
            media_type: deno_ast::MediaType::TypeScript,
            capture_tokens: true,
            scope_analysis: false,
            maybe_syntax: None,
        })
    }

    fn formatted(&self, config: Option<&Configuration>) -> anyhow::Result<String> {
        match config {
            Some(config) => Ok(format_parsed_source(self, config)?.unwrap_or(String::new())),
            None => {
                let config = ConfigurationBuilder::new()
                    .indent_width(2)
                    .line_width(80)
                    .prefer_hanging(true)
                    .prefer_single_line(false)
                    .quote_style(QuoteStyle::PreferSingle)
                    .next_control_flow_position(NextControlFlowPosition::SameLine)
                    .build();

                Ok(format_parsed_source(self, &config)?.unwrap_or(String::new()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_source_from_string() -> anyhow::Result<()> {
        let ts: TS = TS::from_source("let a = 1; let b = 2;".to_string())?;

        let output = ts.formatted(None)?;

        println!("output:");
        println!("{}", output);

        assert_eq!(output.as_str(), "let a = 1;\nlet b = 2;\n");

        Ok(())
    }
}
