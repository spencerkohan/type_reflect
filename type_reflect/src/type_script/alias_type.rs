use crate::AliasType;

use super::to_ts_type;

pub fn emit_alias_type<T>() -> String
where
    T: AliasType,
{
    return format!(
        r#"

export type {alias} = {source};

"#,
        alias = T::name(),
        source = to_ts_type(&T::source_type())
    );
}
