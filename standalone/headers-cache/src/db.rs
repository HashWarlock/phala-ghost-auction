use crate::BlockNumber;

use anyhow::Result;
use rocksdb::DB;
use std::mem::size_of;

pub struct CacheDB(DB);

fn mk_key(prefix: u8, block_number: BlockNumber) -> [u8; size_of::<BlockNumber>() + 1] {
    let mut key = [prefix; size_of::<BlockNumber>() + 1];
    key[1..].copy_from_slice(&block_number.to_be_bytes());
    key
}

impl CacheDB {
    pub fn open(path: &str) -> Result<Self> {
        Ok(CacheDB(DB::open_default(path)?))
    }

    pub fn flush(&self) -> Result<()> {
        self.0.flush()?;
        Ok(())
    }

    fn get(&self, prefix: u8, block: BlockNumber) -> Option<Vec<u8>> {
        self.0.get(&mk_key(prefix, block)).ok().flatten()
    }

    fn put(&self, prefix: u8, block: BlockNumber, value: &[u8]) -> Result<()> {
        self.0.put(&mk_key(prefix, block), value)?;
        Ok(())
    }

    pub fn get_header(&self, block: BlockNumber) -> Option<Vec<u8>> {
        self.get(b'h', block)
    }

    pub fn put_header(&self, block: BlockNumber, value: &[u8]) -> Result<()> {
        self.put(b'h', block, value)
    }

    pub fn get_para_header(&self, block: BlockNumber) -> Option<Vec<u8>> {
        self.get(b'p', block)
    }

    pub fn put_para_header(&self, block: BlockNumber, value: &[u8]) -> Result<()> {
        self.put(b'p', block, value)
    }

    pub fn get_storage_changes(&self, block: BlockNumber) -> Option<Vec<u8>> {
        self.get(b'c', block)
    }

    pub fn put_storage_changes(&self, block: BlockNumber, value: &[u8]) -> Result<()> {
        self.put(b'c', block, value)
    }

    pub fn get_genesis(&self, block: BlockNumber) -> Option<Vec<u8>> {
        self.get(b'g', block)
    }

    pub fn put_genesis(&self, block_number: BlockNumber, value: &[u8]) -> Result<()> {
        self.put(b'g', block_number, value)
    }
}
