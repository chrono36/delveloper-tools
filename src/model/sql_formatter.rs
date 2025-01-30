use sqlformat::{FormatOptions, Indent, QueryParams};

pub struct SqlFormatter {}

impl SqlFormatter {
    pub fn formatter(text: &str) -> Result<String, &'static str> {
        let options = FormatOptions {
            indent: Indent::Tabs,
            uppercase: Some(true),
            lines_between_queries: 2,
            ignore_case_convert: None,
        };

        Ok(sqlformat::format(text, &QueryParams::None, &options))
    }
}
