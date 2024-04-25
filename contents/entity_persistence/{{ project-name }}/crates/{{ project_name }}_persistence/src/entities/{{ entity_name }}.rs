{% import "macros/rust" as rust -%}
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "{{ entity_name }}")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
{%- for field_key in fields -%}
{%- set field = fields[field_key] %}
    pub {{ field["field_name"] }}: {{ rust.field_rust_type(field) }},
{%- endfor %}
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
