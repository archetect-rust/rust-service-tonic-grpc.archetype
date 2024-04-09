use crate::settings::ServerSettings;
use anyhow::Result;
use {{ project_name }}_core::{
    proto::{{ project_name }}_server::{{ ProjectName }}Server as {{ ProjectName }}ProtoServer,
    {{ ProjectName }}Core,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;

use tonic::transport::Server;

pub mod settings;

#[derive(Clone)]
pub struct {{ ProjectName }}Server {
    core: {{ ProjectName }}Core,
    service_port: u16,
    listener: Arc<Mutex<Option<TcpListener>>>,
}

pub struct Builder {
    settings: ServerSettings,
    core: {{ ProjectName }}Core,
}

impl Builder {
    pub fn new(core: {{ ProjectName }}Core) -> Builder {
        Builder {
            settings: ServerSettings::default(),
            core,
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Builder {
        self.settings = settings.clone();
        self
    }

    pub fn with_random_port(mut self) -> Builder {
        self.settings.service_mut().set_port(0);
        self
    }

    pub async fn build(self) -> Result<{{ ProjectName }}Server> {
        let listener = TcpListener::bind((self.settings.host(), self.settings.service().port())).await?;
        let addr = listener.local_addr()?;

        Ok({{ ProjectName }}Server {
            core: self.core,
            service_port: addr.port(),
            listener: Arc::new(Mutex::new(Some(listener))),
        })
    }
}

impl {{ ProjectName }}Server {
    pub fn builder(core: {{ ProjectName }}Core) -> Builder {
        Builder::new(core)
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub async fn serve(&self) -> Result<()> {
        let listener = self.listener.lock().await.take().expect("Listener Expected");

        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<{{ ProjectName }}ProtoServer<{{ ProjectName }}Core>>()
            .await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set({{ project_name }}_core::proto::FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        let server = Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service({{ ProjectName }}ProtoServer::new(self.core.clone()));

        tracing::info!("{{ ProjectName }} started on {}", listener.local_addr()?);

        server.serve_with_incoming(TcpListenerStream::new(listener)).await?;

        Ok(())
    }
}
