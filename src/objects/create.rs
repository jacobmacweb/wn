// Copyright 2024 Miner
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use std::io::Write;

use super::{resolve_filepath, Header};

fn create_header(header: &Header) -> String {
    match header {
        Header::File(size) => format!("file {}", size),
        Header::Dir(size) => format!("dir {}", size),
        Header::Commit(hash) => format!("commit {}", hash),
    }
}

fn encode_object(data: Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    if let Err(e) = e.write_all(&data) {
        panic!("Failed to compress object: {:?}", e);
    }

    let result = e.finish();
    match result {
        Ok(data) => data,
        Err(e) => panic!("Failed to compress object: {:?}", e),
    }
}

// Create a new object
pub fn create_plain_object(header: &Header, data: &[u8]) -> String {
    let header = create_header(header);
    // add newline
    let header = format!("{}\n", header);
    let full_body = header
        .as_bytes()
        .to_vec()
        .into_iter()
        .chain(data.iter().copied())
        .collect::<Vec<u8>>();

    let mut hasher = Sha256::new();
    hasher.update(&full_body);
    let hash = format!("{:x}", hasher.finalize());

    let path = resolve_filepath(&hash);

    // ensure files are created with the correct permissions
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create object directory");
    std::fs::write(&path, encode_object(full_body)).expect("Failed to write object");

    hash
}

pub fn create_file_object(data: &[u8]) -> String {
    create_plain_object(&Header::File(data.len()), data)
}
