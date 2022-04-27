use std::env;
use std::path::PathBuf;

fn main() {
    tonic_build::compile_protos("proto/chat_message.proto").unwrap();
}
