use crate::AliasType;

use super::to_zod_type;

pub fn emit_alias_type<T>() -> String
where
    T: AliasType,
{
    return format!(
        r#"

export const {name}Schema = {schema};
export type {name} = z.infer<typeof {name}Schema>;

"#,
        name = T::name(),
        schema = to_zod_type(&T::source_type())
    );
}
