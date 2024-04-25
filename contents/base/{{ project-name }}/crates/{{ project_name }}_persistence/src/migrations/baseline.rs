{% import "macros/rust" as rust -%}
use sea_schema::migration::{sea_query::*, *};

use crate::{entities, DbResult};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "baseline"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> DbResult<()> {
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
        manager
            .create_table(
                Table::create()
                    .table(entities::{{ entity["entity_name"] }}::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entities::{{ entity["entity_name"] }}::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_owned()),
                    )
{%- for field_key in entity.fields -%}
{%- set field = entity.fields[field_key] %}
                    .col(
                        {{ rust.field_column_definition(entity, field) }}
                    )
{%- endfor %}
                    .to_owned(),
            ).await?;
        {%- endfor %}

        Ok(())
     }

    async fn down(&self, manager: &SchemaManager) -> DbResult<()> {
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
        manager
            .drop_table(
                Table::drop()
                    .table(entities::{{ entity["entity_name"] }}::Entity)
                    .if_exists()
                    .to_owned(),
            ).await?;
        {% endfor %}

        Ok(())
    }
}
