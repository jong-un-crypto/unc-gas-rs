use crate::UncGas;

impl schemars::JsonSchema for UncGas {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}
