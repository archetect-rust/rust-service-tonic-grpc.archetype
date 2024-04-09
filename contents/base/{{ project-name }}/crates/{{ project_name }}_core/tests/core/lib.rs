use anyhow::Result;
use {{ project_name }}_core::proto::{{ project_name }}_server::{{ ProjectName }};
use {{ project_name }}_core::proto::{
    Create{{ ProjectPrefix }}Request, Create{{ ProjectPrefix }}Response, Get{{ ProjectPrefix }}ListRequest,
    Get{{ ProjectPrefix }}ListResponse,
};
use {{ project_name }}_core::{{ ProjectName }}Core;
use {{ project_name }}_persistence::{{ ProjectName }}Persistence;
use tonic::Request;

#[tokio::test]
async fn test_create_{{ project_prefix }}() -> Result<()> {
    let core = core().await?;

    let response = core
        .get_{{ project_prefix }}_list(Request::new(Get{{ ProjectPrefix }}ListRequest { page_size: 0, page: 0 }))
        .await?;
    let Get{{ ProjectPrefix }}ListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let response = core
        .create_{{ project_prefix }}(Request::new(Create{{ ProjectPrefix }}Request {
            contents: "Contents".to_string(),
        }))
        .await?;
    let Create{{ ProjectPrefix }}Response { record } = response.into_inner();
    let record = record.expect("Record Expected");
    assert_eq!(&record.contents, "Contents");

    let response = core
        .get_{{ project_prefix }}_list(Request::new(Get{{ ProjectPrefix }}ListRequest { page_size: 0, page: 0 }))
        .await?;
    let Get{{ ProjectPrefix }}ListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    Ok(())
}

async fn core() -> Result<{{ ProjectName }}Core> {
    let persistence = {{ ProjectName }}Persistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = {{ ProjectName }}Core::builder(persistence)
        .build()
        .await?;
    Ok(core)
}
