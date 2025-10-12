use std::collections::HashMap;

use crate::pubsub::v2::{DatumType, Relation};
pub(super) trait Lookup {
    fn lookup_relations_fields_type(&self, table_id: &str, field_id: &str) -> Option<DatumType>;
}

impl<const SIZE: usize> Lookup for HashMap<String, Relation<SIZE>> {
    fn lookup_relations_fields_type(&self, table_id: &str, field_id: &str) -> Option<DatumType> {
        let relation = self.get(table_id)?;
        relation.lookup_feilds_type(field_id)
    }
}
