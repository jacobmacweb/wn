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

use std::io::Read;

use flate2::read::ZlibDecoder;

use super::{resolve_filepath, Header};

fn decode_object(data: Vec<u8>) -> Vec<u8> {
    let mut d = ZlibDecoder::new(&data[..]);
    let mut result = Vec::new();
    if let Err(e) = d.read_to_end(&mut result) {
        panic!("Failed to decompress object: {:?}", e);
    }

    result
}

// Read an object
fn read_object(hash: &str) -> Vec<u8> {
    let path = resolve_filepath(hash);
    let data = match std::fs::read(&path) {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to read object at {:?}: {:?}", path, e);
        }
    };

    decode_object(data)
}

pub fn read_plain_object(hash: &str) -> (Header, Vec<u8>) {
    let raw = read_object(hash);

    // Go until first newline
    let mut header_end = 0;
    for (i, &b) in raw.iter().enumerate() {
        if b == b'\n' {
            header_end = i;
            break;
        }
    }

    let header = std::str::from_utf8(&raw[..header_end])
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let header = match header.as_slice() {
        ["file", size] => Header::File(size.parse().unwrap()),
        ["dir", size] => Header::Dir(size.parse().unwrap()),
        ["commit", hash] => Header::Commit(hash.to_string()),
        _ => panic!("Invalid object header"),
    };

    (header, raw[header_end + 1..].to_vec())
}

#[allow(dead_code)]
pub fn read_file_object(hash: &str) -> Vec<u8> {
    let (header, data) = read_plain_object(hash);
    match header {
        Header::File(_) => data,
        _ => panic!("Invalid object type"),
    }
}
