{%- for entity_key in model.entities -%}
{%- set entity = model.entities[entity_key] -%}
pub mod {{ entity["entity_name"] }};
{%- endfor %}
