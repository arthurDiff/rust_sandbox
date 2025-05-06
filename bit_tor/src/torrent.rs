use std::path::PathBuf;

use chrono::{DateTime, Utc};

pub struct FileItem {
    pub path: PathBuf,
    pub size: i64,
    pub offset: i64,
}

pub struct Tracker {
    pub addr: String,
}

pub struct Torrent {
    name: String,
    is_public: Option<bool>,
    files: Vec<FileItem>,
    file_dir: String,
    download_dir: String,
    trackers: Vec<Tracker>,
    block_size: i32,
    piece_size: i32,
    piece_hashes: Vec<Vec<u8>>,
    verified_pieces: Vec<bool>,
    acuired_blocks: Vec<Vec<bool>>,
    pub uploaded: i64,
    pub comment: String,
    pub created_by: String,
    pub creation_date: DateTime<Utc>,
}

impl Torrent {
    fn new() -> Self {
        todo!()
    }
    pub fn total_size(&self) -> i64 {
        self.files.iter().map(|f| f.size).sum()
    }
    pub fn downloaded(&self) -> i64 {
        self.piece_size as i64 * self.verified_pieces.iter().filter(|p| **p).count() as i64
    }
    pub fn remainder(&self) -> i64 {
        self.total_size() - self.downloaded()
    }
    pub fn get_piece_size(&self, piece_idx: usize) -> i32 {
        if piece_idx != self.piece_hashes.len() {
            return self.piece_size;
        };
        match self.total_size() % self.piece_size as i64 {
            v if v > 0 => v as i32,
            _ => self.piece_size,
        }
    }
}
