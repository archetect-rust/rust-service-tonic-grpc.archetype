use anyhow::Result;
use {{ project_name }}_persistence::{entities::*, sea_orm::prelude::Uuid, sea_orm::ActiveValue};
use std::str::FromStr;
use tonic::Status;

use crate::proto::*;

pub trait ConvertTo<T>: Sized {
    fn convert_to(self) -> Result<T, Status>;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}
impl ConvertFrom<{{ entity["entity_name"] }}::Model> for {{ entity["EntityName"] }} {
    fn convert_from(value: {{ entity["entity_name"] }}::Model) -> Self {
        {{ entity["EntityName"] }} {
            id: Some(Id {
                value: value.id.to_string(),
            }),
            contents: value.contents,
        }
    }
}

impl ConvertTo<{{ entity["entity_name"] }}::ActiveModel> for {{ entity["EntityName"] }} {
    fn convert_to(self) -> std::result::Result<{{ entity["entity_name"] }}::ActiveModel, Status> {
        Ok({{ entity["entity_name"] }}::ActiveModel {
            id: ActiveValue::Set(self.id.convert_to()?),
            contents: ActiveValue::Set(self.contents),
        })
    }
}
{%- endfor %}

impl ConvertTo<Uuid> for Option<Id> {
    fn convert_to(self) -> Result<Uuid, Status> {
        match self {
            None => Err(Status::invalid_argument("Id is required".to_string())),
            Some(id) => id.convert_to(),
        }
    }
}

impl ConvertTo<Uuid> for Id {
    fn convert_to(self) -> Result<Uuid, Status> {
        Uuid::from_str(self.value.as_str())
            .map_err(|_| Status::invalid_argument("Id was not set to a valid UUID".to_string()))
    }
}
