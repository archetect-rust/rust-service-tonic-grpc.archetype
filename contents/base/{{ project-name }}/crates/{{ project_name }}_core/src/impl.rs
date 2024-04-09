use tonic::{Request, Response, Status};

use {{ project_name }}_persistence::{entities::*, sea_orm::prelude::*, sea_orm::*, Page};

use crate::{
    {{ ProjectName }}Core,
    proto::{
        Create{{ ProjectPrefix }}Request, Create{{ ProjectPrefix }}Response,
        {{ project_name }}_server::{{ ProjectName }}, Get{{ ProjectPrefix }}ListRequest, Get{{ ProjectPrefix }}ListResponse,
    },
};
use crate::conversion::{ConvertFrom, ConvertTo};
use crate::proto::{{'{'}}{{ ProjectPrefix }}, Find{{ ProjectPrefix }}Request, Find{{ ProjectPrefix }}Response, Update{{ ProjectPrefix }}Request, Update{{ ProjectPrefix }}Response};

#[tonic::async_trait]
impl {{ ProjectName }} for {{ ProjectName }}Core {
    async fn find_{{ project_prefix }}(&self, request: Request<Find{{ ProjectPrefix }}Request>) -> Result<Response<Find{{ ProjectPrefix }}Response>, Status> {
        let request = request.into_inner();
        let id = request.id.convert_to()?;
        let result = self.persistence.find_{{ project_prefix }}(id).await;
        match result {
            Ok(result) => {
                match result {
                    None => Err(Status::not_found("Record not found".to_owned())),
                    Some(model) => {
                        Ok(Response::new(Find{{ ProjectPrefix }}Response {
                            record: Some({{ ProjectPrefix }}::convert_from(model))
                        }))
                    }
                }
            }
            Err(err) => {
                match err {
                    DbErr::RecordNotFound(err) => Err(Status::not_found(err)),
                    _ => Err(Status::internal("Unexpected error")),
                }
            }
        }
    }

    async fn create_{{ project_prefix }}(
        &self,
        request: Request<Create{{ ProjectPrefix }}Request>,
    ) -> Result<Response<Create{{ ProjectPrefix }}Response>, Status> {
        let request = request.into_inner();
        tracing::info!("Received: {:?}", request);

        let {{ project_prefix }}_record = {{ project_prefix }}::ActiveModel {
            id: Set(Uuid::new_v4()),
            contents: Set(request.contents),
        };

        let result = self.persistence.insert_{{ project_prefix }}({{ project_prefix }}_record).await;
        if let Ok(entity) = result {
            return Ok(Response::new(Create{{ ProjectPrefix }}Response {
                record: Some({{ ProjectPrefix }}::convert_from(entity)),
            }));
        }

        Err(Status::internal("Unexpected Error"))
    }

    async fn update_{{ project_prefix }}(
        &self,
        request: Request<Update{{ ProjectPrefix }}Request>,
    ) -> Result<Response<Update{{ ProjectPrefix }}Response>, Status> {
        let {{ project_prefix }}_record: {{ project_prefix }}::ActiveModel =
            request.into_inner().record.unwrap().convert_to()?;
        let result = self
            .persistence
            .update_{{ project_prefix }}({{ project_prefix }}_record.into_active_model())
            .await;

        match result {
            Ok(entity) => Ok(Response::new(Update{{ ProjectPrefix }}Response {
                record: Some({{ ProjectPrefix }}::convert_from(entity)),
            })),
            Err(err) => match err {
                DbErr::RecordNotFound(err) => Err(Status::not_found(err)),
                _ => Err(Status::internal("Unexpected error")),
            },
        }
    }

    async fn get_{{ project_prefix }}_list(
        &self,
        request: Request<Get{{ ProjectPrefix }}ListRequest>,
    ) -> Result<Response<Get{{ ProjectPrefix }}ListResponse>, Status> {
        let request = request.into_inner();
        tracing::info!("Received: {:?}", request);

        let response = self
            .persistence
            .get_{{ project_prefix }}_list(request.page_size as usize, request.page as usize)
            .await;

        match response {
            Ok(Page { records, total_pages }) => {
                let records = records.into_iter().map({{ ProjectPrefix }}::convert_from).collect();
                Ok(Response::new(Get{{ ProjectPrefix }}ListResponse {
                    records: records,
                    total_pages: total_pages as u32,
                }))
            }
            Err(_) => Err(Status::internal("Unknown Error")),
        }
    }
}
