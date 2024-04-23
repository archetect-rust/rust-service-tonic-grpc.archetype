pub mod proto {
    tonic::include_proto!("{{ project_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ project_name }}");
}
