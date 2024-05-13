use std::{path::Path, sync::Arc};

use kira::sound::{streaming::StreamingSoundHandle, FromFileError};

// #[derive(Debug, Clone)]
pub struct NoiseTrack {
    pub name: String,
    pub path: String,
    pub volume_level: f32,
    pub handle: Arc<StreamingSoundHandle<FromFileError>>,
}

pub fn get_stem(name: &Path) -> String {
    Path::file_stem(name).unwrap().to_str().unwrap().to_string()
}
pub fn load_data() -> Vec<NoiseTrack> {
    let mut files = vec![];
    if !files.is_empty() {
        files.clear();
    }
    for entry in walkdir::WalkDir::new("assets/sounds/") {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            files.push(NoiseTrack {
                name: get_stem(entry.path()),
                path: entry.path().to_str().unwrap().to_string(),
                volume_level: 0.5,
                handle: todo!(),
            });
        }
    }
    files
}
