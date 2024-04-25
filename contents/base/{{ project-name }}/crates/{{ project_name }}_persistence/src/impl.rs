use crate::sea_orm::entity::prelude::*;
use crate::{{'{'}}{{ ProjectName }}Persistence, DbResult};

use crate::entities::*;
use crate::page::Page;

impl {{ ProjectName }}Persistence {
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
    pub async fn find_{{ entity["entity_name"] }}(
        &self,
        id: Uuid,
    ) -> DbResult<Option<{{ entity["entity_name"] }}::Model>> {
        let record = {{ entity["entity_name"] }}::Entity::find_by_id(id).one(self.connection()).await?;
        Ok(record)
    }

    pub async fn insert_{{ entity["entity_name"] }}(
        &self,
        {{ entity["entity_name"] }}_record: {{ entity["entity_name"] }}::ActiveModel,
    ) -> DbResult<{{ entity["entity_name"] }}::Model> {
        let result = {{ entity["entity_name"] }}_record.insert(self.connection()).await?;
        Ok(result)
    }

    pub async fn update_{{ entity["entity_name"] }}(
        &self,
        {{ entity["entity_name"] }}_record: {{ entity["entity_name"] }}::ActiveModel,
    ) -> DbResult<{{ entity["entity_name"] }}::Model> {
        let result = {{ entity["entity_name"] }}_record.update(self.connection()).await?;
        Ok(result)
    }

    pub async fn get_{{ entity["entity_name"] }}_list(
        &self,
        page_index: u32,
        page_size: u32,
    ) -> DbResult<Page<{{ entity["entity_name"] }}::Model>> {
        let page_size = page_size.min(100);
        let paginator =
            {{ entity["entity_name"] }}::Entity::find().paginate(self.connection(), page_size as usize);

        let records = paginator.fetch_page(page_index as usize).await?;
        let total_records = paginator.num_items().await? as u64;

        Ok(Page::new(records, page_index, page_size, total_records))
    }

    {%- endfor %}
}
