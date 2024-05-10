use anyhow::Result;
use {{ project_name }}_persistence::{entities::*, sea_orm::prelude::Uuid, sea_orm::ActiveValue};
use std::str::FromStr;
use tonic::Status;

use crate::proto::*;

pub trait ConvertTo<T>: Sized {
    fn convert_to(self) -> T;
}

pub trait TryConvertTo<T, E>: Sized {
    fn try_convert_to(self) -> Result<T, E>;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}

pub trait TryConvertFrom<T: Sized, E> {
    fn try_convert_from(value: T) -> Result<T, E>;
}
{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] %}

impl ConvertFrom<{{ entity["entity_name"] }}::Model> for {{ entity["EntityName"] }} {
    fn convert_from(value: {{ entity["entity_name"] }}::Model) -> Self {
        {{ entity["EntityName"] }} {
            id: Some(value.id.to_string()),
{%- for field_key in entity.fields -%}
{%- set field = entity.fields[field_key] %}
            {{ field["field_name"] }} : value.{{ field["field_name"] }},
{%- endfor %}
        }
    }
}

impl TryConvertTo<{{ entity["entity_name"] }}::ActiveModel, Status> for {{ entity["EntityName"] }} {
    fn try_convert_to(self) -> std::result::Result<{{ entity["entity_name"] }}::ActiveModel, Status> {
        let id = self.id.try_convert_to()?;
        Ok({{ entity["entity_name"] }}::ActiveModel {
            id: id.map(|id| ActiveValue::Set(id)).unwrap_or( ActiveValue::NotSet),
{%- for field_key in entity.fields -%}
{%- set field = entity.fields[field_key] %}
            {{ field["field_name"] }}: ActiveValue::Set(self.{{ field["field_name"] }}),
{%- endfor %}
        })
    }
}
{%- endfor %}

impl TryConvertTo<Option<Uuid>, Status> for Option<String> {
    fn try_convert_to(self) -> Result<Option<Uuid>, Status> {
        match self {
            None => Ok(None),
            Some(id) => Ok(Some(id.try_convert_to()?)),
        }
    }
}

impl TryConvertTo<Uuid, Status> for String {
    fn try_convert_to(self) -> Result<Uuid, Status> {
        Uuid::from_str(self.as_str())
            .map_err(|_| Status::invalid_argument("Id was not set to a valid UUID".to_string()))
    }
}
