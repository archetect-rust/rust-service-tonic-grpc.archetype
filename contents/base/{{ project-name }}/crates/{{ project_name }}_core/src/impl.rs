{% import "macros/rust" as rust -%}
use tonic::{Request, Response, Status};
use tracing::info;

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
    async fn create_{{ entity["entity_name"] }}(&self, request: Request<{{ entity["EntityName"] }}>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let {{ entity["entity_name"] }} = request.into_inner();
        info!("Creating: {:?}", {{ entity["entity_name"] }});

        self.persistence.insert_{{ entity["entity_name"] }}({{ entity["entity_name"] }}.convert_to()?)
            .await
            .map({{ entity["EntityName"] }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }

    async fn get_{{ entity["entity_name"] }}(&self, request: Request<Get{{ entity["EntityName"] }}Request>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let request = request.into_inner();
        info!("Getting {{ entity["EntityName"] }}: {:?}", request);
        let id = request.id.convert_to()?;

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
        info!("Getting {{ entity["EntityName"] | pluralize }}: {:?}", request);

        let response = self
            .persistence
            .get_{{ entity["entity_name"] }}_list(request.page_index, request.page_size)
            .await;

        match response {
            Ok(Page { records, index, next, has_next, previous, has_previous, total, total_records }) => {
                let records = records.into_iter().map({{ entity["EntityName"] }}::convert_from).collect();
                Ok(Response::new(Get{{ entity["EntityName"] | pluralize }}Response {
                    records,
                    index,
                    next,
                    has_next,
                    previous,
                    has_previous,
                    total,
                    total_records,
                }))
            }
            Err(err) => Err(Status::internal(format!("Database Error: '{err}'"))),
        }
    }

    async fn update_{{ entity["entity_name"] }}(&self, request: Request<{{ entity["EntityName"] }}>) -> Result<Response<{{ entity["EntityName"] }}>, Status> {
        let {{ entity["entity_name"] }} = request.into_inner();
        info!("Updating: {:?}", {{ entity["entity_name"] }});

        if {{ entity["entity_name"] }}.id.is_none() {
            return Err(Status::invalid_argument("{{ entity["entity_name"] }} id is required"));
        }

        self.persistence.update_{{ entity["entity_name"] }}({{ entity["entity_name"] }}.convert_to()?)
            .await
            .map({{ entity["EntityName"] }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }
{%- endfor %}
}
