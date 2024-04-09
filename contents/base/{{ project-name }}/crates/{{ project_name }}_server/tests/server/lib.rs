use anyhow::Result;
use {{ project_name }}_client::proto::{{ project_name }}_client::{{ ProjectName }}Client;
use {{ project_name }}_client::proto::Create{{ ProjectPrefix }}Request;
use {{ project_name }}_core::{{ ProjectName }}Core;
use {{ project_name }}_persistence::{{ ProjectName }}Persistence;
use {{ project_name }}_server::{{ ProjectName }}Server;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::test]
async fn test_core() -> Result<()> {
    let (mut client, _) = init().await?;

    let request = Request::new(Create{{ ProjectPrefix }}Request {
        contents: "Contents".to_string(),
    });

    let response = client.create_{{ project_prefix }}(request).await?;
    let response = response.into_inner();
    assert_eq!(response.record.unwrap().contents, "Contents".to_owned());

    Ok(())
}

async fn init() -> Result<({{ ProjectName }}Client<Channel>, {{ ProjectName }}Server)> {
    let persistence = {{ ProjectName }}Persistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = {{ ProjectName }}Core::builder(persistence)
        .build()
        .await?;
    let server = {{ ProjectName }}Server::builder(core)
        .with_random_port()
        .build()
        .await?;

    let server_clone = server.clone();

    tokio::spawn(async move {
        let _ = server_clone.serve().await;
    });

    let addr = format!("http://localhost:{}", server.service_port());
    let client = {{ ProjectName }}Client::connect(addr).await?;

    Ok((client, server))
}
