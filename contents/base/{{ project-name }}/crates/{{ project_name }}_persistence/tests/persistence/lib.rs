use anyhow::Result;
use {{ project_name }}_persistence::entities::*;
use {{ project_name }}_persistence::sea_orm::prelude::*;
use {{ project_name }}_persistence::sea_orm::*;
use {{ project_name }}_persistence::{{'{'}}{{ ProjectName }}Persistence, DbResult, Page};

#[tokio::test]
async fn test_insert_{{ project_prefix }}() -> Result<()> {
    let persistence = persistence().await?;

    let {{ project_prefix }} = insert_{{ project_prefix }}(&persistence).await?;
    assert_eq!(&{{ project_prefix }}.contents, "Hello, World!");

    println!("{:?}", {{ project_prefix }});
    Ok(())
}

#[tokio::test]
async fn test_update_{{ project_prefix }}() -> Result<()> {
    let persistence = persistence().await?;

    let {{ project_prefix }} = insert_{{ project_prefix }}(&persistence).await?;
    assert_eq!(&{{ project_prefix }}.contents, "Hello, World!");

    let mut {{ project_prefix }} = {{ project_prefix }}.into_active_model();
    {{ project_prefix }}.contents = Set("Goodbye, World!".to_owned());
    let {{ project_prefix }} = persistence.update_{{ project_prefix }}({{ project_prefix }}).await?;
    assert_eq!(&{{ project_prefix }}.contents, "Goodbye, World!");

    println!("{:?}", {{ project_prefix }});
    Ok(())
}

#[tokio::test]
async fn test_list_{{ project_prefix }}s() -> Result<()> {
    let persistence = persistence().await?;

    let Page { records, total_pages } = persistence.get_{{ project_prefix }}_list(10, 0).await?;
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let _ = insert_{{ project_prefix }}(&persistence).await?;
    let Page { records, total_pages } = persistence.get_{{ project_prefix }}_list(10, 0).await?;
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    for _ in 1..=14 {
        let _ = insert_{{ project_prefix }}(&persistence).await?;
    }
    let Page { records, total_pages } = persistence.get_{{ project_prefix }}_list(10, 0).await?;
    assert_eq!(records.len(), 10);
    assert_eq!(total_pages, 2);

    let Page { records, total_pages } = persistence.get_{{ project_prefix }}_list(10, 1).await?;
    assert_eq!(records.len(), 5);
    assert_eq!(total_pages, 2);

    Ok(())
}

async fn insert_{{ project_prefix }}(persistence: &{{ ProjectName }}Persistence) -> DbResult<{{ project_prefix }}::Model> {
    let {{ project_prefix }}_record = {{ project_prefix }}::ActiveModel {
        id: Set(Uuid::new_v4()),
        contents: Set("Hello, World!".to_owned()),
    };

    persistence.insert_{{ project_prefix }}({{ project_prefix }}_record).await
}

async fn persistence() -> Result<{{ ProjectName }}Persistence> {
    {{ ProjectName }}Persistence::builder()
        .with_temp_db()
        .build()
        .await
}
