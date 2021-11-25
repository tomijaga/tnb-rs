pub fn format_node_url(protocol: &str, host: &str, port: u16) -> String {
    format!("{}://{}:{}", protocol, host, port)
}
