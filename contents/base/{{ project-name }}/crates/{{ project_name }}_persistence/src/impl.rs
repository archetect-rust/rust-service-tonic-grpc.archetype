use crate::sea_orm::entity::prelude::*;
use crate::Page;
use crate::{{'{'}}{{ ProjectName }}Persistence, DbResult};

use crate::entities::*;

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
        page_size: usize,
        page: usize,
    ) -> DbResult<Page<{{ project_prefix }}::Model>> {
        let paginator =
            {{ project_prefix }}::Entity::find().paginate(self.connection(), if page_size > 0 { page_size } else { 10 });

        let records = paginator.fetch_page(page).await?;
        let total_pages = paginator.num_pages().await?;

        Ok(Page { records, total_pages })
    }
}
