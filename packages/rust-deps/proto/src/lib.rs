pub mod posts {
    tonic::include_proto!("posts");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
}
