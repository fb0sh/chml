mod download;
pub use download::download_fpc_client;
pub mod print;
mod random;
pub use random::random_node;
pub use random::random_port;
pub use random::random_string;
pub fn color(text: &str, code: u8) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, text)
}
