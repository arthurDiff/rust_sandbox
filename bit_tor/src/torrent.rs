use std::path::PathBuf;

use chrono::{DateTime, Utc};

pub struct FileItem {
    pub path: PathBuf,
    pub size: u32,
    pub offset: u32,
}

pub struct Tracker {
    pub addr: String,
}

#[allow(dead_code)]
pub struct Torrent {
    name: String,
    is_public: bool,
    files: Vec<FileItem>,
    file_dir: String,
    download_dir: PathBuf,
    trackers: Vec<Tracker>,
    block_size: u32,
    piece_size: u32,
    piece_hashes: Vec<Vec<u8>>,
    verified_pieces: Vec<bool>,
    acuired_blocks: Vec<Vec<bool>>,
    pub uploaded: u64,
    pub comment: String,
    pub created_by: String,
    pub creation_date: DateTime<Utc>,
}

impl Torrent {
    pub fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size as u64).sum()
    }
    pub fn downloaded(&self) -> u64 {
        self.piece_size as u64 * self.verified_pieces.iter().filter(|p| **p).count() as u64
    }

    pub fn remainder(&self) -> u64 {
        self.total_size() - self.downloaded()
    }

    pub fn block_count(&self, piece_idx: usize) -> u32 {
        self.piece_size(piece_idx) / self.block_size
    }

    pub fn piece_size(&self, piece_idx: usize) -> u32 {
        if piece_idx != self.piece_hashes.len() {
            return self.piece_size;
        };
        match self.total_size() % self.piece_size as u64 {
            v if v > 0 => v as u32,
            _ => self.piece_size,
        }
    }
    pub fn block_size(&self, piece_idx: usize, block_idx: usize) -> u32 {
        if block_idx != self.block_count(piece_idx) as usize - 1 {
            return self.block_size;
        }

        match self.piece_size(piece_idx) % self.block_size {
            v if v > 0 => v,
            _ => self.block_size,
        }
    }
}
