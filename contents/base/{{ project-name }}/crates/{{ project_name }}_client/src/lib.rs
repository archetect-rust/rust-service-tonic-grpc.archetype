pub mod proto {
    tonic::include_proto!("{{ project_prefix }}.service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ project_prefix }}.service");
}
