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

use std::path::PathBuf;

use log::{debug, error};

use crate::index::resolve_dir;

/*
* Create an empty objects folder
*/
pub fn create_empty(dir: Option<&PathBuf>) {
    // Create empty directories
    let mut root = resolve_dir(dir);
    root = root.join("refs");

    match std::fs::create_dir_all(&root) {
        Ok(_) => {
            debug!("Created empty refs directory at {:?}", root);
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    error!("Refs directory already exists at {:?}", root);
                }
                _ => {
                    error!("Failed to create refs directory at {:?}: {:?}", root, e);
                }
            }

            std::process::exit(1);
        }
    }
}
