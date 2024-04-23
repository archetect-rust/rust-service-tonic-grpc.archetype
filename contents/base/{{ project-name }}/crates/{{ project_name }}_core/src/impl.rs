{% import "macros/rust" as rust -%}
use tonic::{Request, Response, Status};

use {{ project_name }}_persistence::Page;

use crate::{{ ProjectName }}Core;
use crate::conversion::{ConvertFrom, ConvertTo};
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
{{ rust.entity_proto_imports(entity) }}{% endfor %}
use crate::proto::{{ project_name }}_server::{{ ProjectName }};

#[tonic::async_trait]
impl {{ ProjectName }} for {{ ProjectName }}Core {
{% for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
    async fn create_{{ entity["entity_name"] }}(&self, request: Request<Create{{ entity["EntityName"] }}Request>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let {{ entity["entity_name"] }} = request
            .into_inner()
            .{{ entity["entity_name"] }}
            .ok_or(Status::invalid_argument("{{ entity["EntityName"] }} missing"))?;

        self.persistence.insert_{{ entity["entity_name"] }}({{ entity["entity_name"] }}.convert_to()?)
            .await
            .map({{ entity["EntityName"] }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }

    async fn get_{{ entity["entity_name"] }}(&self, request: Request<Get{{ entity["EntityName"] }}Request>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let id = request.into_inner().id.convert_to()?;

        self.persistence.find_{{ entity["entity_name"] }}(id)
            .await
            .map_err(|err| {
                match err {
                    err => Status::internal(format!("Error: '{err}'")),
                }
            })?
            .ok_or(Status::not_found(format!("{{ entity["EntityName"] }} not found by id '{id}'")))
            .map({{ entity["EntityName"] }}::convert_from)
            .map(Response::new)
    }

    async fn get_{{ entity["entity_name"] | pluralize }}(&self, request: Request<Get{{ entity["EntityName"] | pluralize }}Request>) -> Result<Response<Get{{ entity["EntityName"] | pluralize }}Response>, Status> {
        let request = request.into_inner();
        tracing::info!("Received: {:?}", request);

        let response = self
            .persistence
            .get_{{ entity["entity_name"] }}_list(request.start_page as usize, request.page_size as usize)
            .await;

        match response {
            Ok(Page { records, index: _, next, has_next, previous, has_previous, total_pages, total_records }) => {
                let records = records.into_iter().map({{ entity["EntityName"] }}::convert_from).collect();
                Ok(Response::new(Get{{ entity["EntityName"] | pluralize }}Response {
                    {{ entity["entity_name"] | pluralize }}: records,
                    has_next,
                    has_previous,
                    next_page: next as i32,
                    previous_page: previous as i32,
                    total_pages: total_pages as i32,
                    total_elements: total_records as i64,
                }))
            }
            Err(err) => Err(Status::internal(format!("Database Error: '{err}'"))),
        }
    }

    async fn update_{{ entity["entity_name"] }}(&self, request: Request<Update{{ entity["EntityName"] }}Request>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let {{ entity["entity_name"] }} = request.into_inner()
            .{{ entity["entity_name"] }}
            .ok_or(Status::invalid_argument("Catalog missing"))?;

        self.persistence.update_{{ entity["entity_name"] }}({{ entity["entity_name"] }}.convert_to()?)
            .await
            .map({{ entity["EntityName"] }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }
{%- endfor %}
}
