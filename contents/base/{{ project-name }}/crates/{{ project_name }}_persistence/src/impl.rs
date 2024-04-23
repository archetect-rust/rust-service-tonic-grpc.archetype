use crate::sea_orm::entity::prelude::*;
use crate::{{'{'}}{{ ProjectName }}Persistence, DbResult};

use crate::entities::*;
use crate::page::Page;

impl {{ ProjectName }}Persistence {
    pub async fn find_{{ project_prefix }}(
        &self,
        id: Uuid,
    ) -> DbResult<Option<{{ project_prefix }}::Model>> {
        let record = {{ project_prefix }}::Entity::find_by_id(id).one(self.connection()).await?;
        Ok(record)
    }

    pub async fn insert_{{ project_prefix }}(
        &self,
        {{ project_prefix }}_record: {{ project_prefix }}::ActiveModel,
    ) -> DbResult<{{ project_prefix }}::Model> {
        let result = {{ project_prefix }}_record.insert(self.connection()).await?;
        Ok(result)
    }

    pub async fn update_{{ project_prefix }}(
        &self,
        {{ project_prefix }}_record: {{ project_prefix }}::ActiveModel,
    ) -> DbResult<{{ project_prefix }}::Model> {
        let result = {{ project_prefix }}_record.update(self.connection()).await?;
        Ok(result)
    }

    pub async fn get_{{ project_prefix }}_list(
        &self,
        index: usize,
        page_size: usize,
    ) -> DbResult<Page<{{ project_prefix }}::Model>> {
        let page_size = page_size.min(100);
        let paginator =
            {{ project_prefix }}::Entity::find().paginate(self.connection(), page_size);

        let records = paginator.fetch_page(index).await?;
        let total_records = paginator.num_items().await?;

        Ok(Page::new(records, index, page_size, total_records))
    }
}
