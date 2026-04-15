use serde::Deserialize;

use diesel::prelude::*;

use sui_indexer_alt_framework::FieldCount;
use sui_types::object::Object;

use crate::models::Freezable;
use crate::schema::indexer::extension_freezes;

#[derive(Deserialize, Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = extension_freezes)]
pub struct StoredExtensionFreeze {
    pub id: String,
    pub owner_id: String,
    pub package_id: String,
    pub module_name: String,
    pub struct_name: String,
}

impl StoredExtensionFreeze {
    pub fn from_object<T: Freezable>(obj: &Object, parent: &T) -> Self {
        Self {
            id: obj.id().to_canonical_string(true),
            owner_id: parent.id(),
            package_id: parent.package_id(),
            module_name: parent.module_name(),
            struct_name: parent.struct_name(),
        }
    }
}
