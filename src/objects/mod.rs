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

pub mod create;
pub mod read;

use std::path::PathBuf;

use log::{debug, error};

use crate::index::resolve_dir;

#[derive(Debug)]
pub enum Header {
    File(usize),    // File size
    Dir(usize),     // Number of files
    Commit(String), // Dir hash
}

fn resolve_filepath(hash: &str) -> PathBuf {
    resolve_dir(None)
        .join("objects")
        .join(&hash[..2])
        .join(&hash[2..])
}

/*
* Create an empty objects folder
*/
pub fn create_empty(dir: Option<&PathBuf>) {
    // Create empty directories
    let mut root = resolve_dir(dir);
    root = root.join("objects");

    match std::fs::create_dir_all(&root) {
        Ok(_) => {
            debug!("Created empty objects directory at {:?}", root);
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    error!("Objects directory already exists at {:?}", root);
                }
                _ => {
                    error!("Failed to create objects directory at {:?}: {:?}", root, e);
                }
            }

            std::process::exit(1);
        }
    }
}
