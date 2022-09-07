use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::path::{Component, Components, Path, PathBuf};

use async_trait::async_trait;
use log::{debug, error, warn};
use streaming_iterator::StreamingIterator;
use tokio::sync::RwLock;

use crate::experiments::array_streaming_iterator::ArrayStreamingIt;
use crate::fs::dir_entry::DirEntry;
use crate::fs::filesystem_front::FilesystemFront;
use crate::fs::fsf_ref::FsfRef;
use crate::fs::read_error::{ListError, ReadError};
use crate::fs::write_error::WriteError;

pub enum Record {
    File(Vec<u8>),
    Dir(HashMap<PathBuf, Record>),
}

impl Record {
    // If creating == true, it creates a Dir, but since it returns a mut ref you can immediately
    // override it with File.
    fn get_mut(&mut self, path: &[Component], creating: bool) -> Option<&mut Record> {
        if path.len() == 0 {
            Some(self)
        } else {
            let first = PathBuf::new().join(path[0]);
            match self {
                Record::File(_) => {
                    None
                }
                Record::Dir(ref mut items) => {
                    if items.contains_key(&first) {
                        return items.get_mut(&first).unwrap().get_mut(&path[1..], creating);
                    }

                    if creating {
                        items.insert(first.clone(), Record::Dir(HashMap::new()));
                        return items.get_mut(&first).unwrap().get_mut(&path[1..], creating);
                    }

                    None
                }
            }
        }
    }

    fn get(&self, path: &[Component]) -> Option<&Record> {
        if path.len() == 0 {
            Some(self)
        } else {
            let first = PathBuf::new().join(path[0]);
            match self {
                Record::File(_) => {
                    None
                }
                Record::Dir(ref items) => {
                    items.get(&first).map(|r| r.get(&path[1..])).flatten()
                }
            }
        }
    }

    fn is_empty_dir(&self) -> bool {
        match &self {
            Record::File(_) => false,
            Record::Dir(contents) => contents.is_empty(),
        }
    }

    fn is_dir(&self) -> bool {
        match &self {
            Record::File(_) => false,
            Record::Dir(contents) => true,
        }
    }

    fn is_file(&self) -> bool {
        !self.is_dir()
    }

    fn create_dir(&mut self, path: &Path) -> bool {
        let components: Vec<Component> = path.components().collect();

        if self.get(&components).is_some() {
            return false;
        }

        self.get_mut(&components, true).is_some()
    }

    fn overwrite_file(&mut self, path: &Path, contents: Vec<u8>) -> bool {
        let components: Vec<Component> = path.components().collect();

        if let Some(rec) = self.get_mut(&components, false) {
            if rec.is_file() {
                *rec = Record::File(contents);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn create_file(&mut self, path: &Path, contents: Vec<u8>) -> bool {
        let components: Vec<Component> = path.components().collect();

        if self.get(&components).is_some() {
            return false;
        }

        self.get_mut(&components, true).map(|maybe_last| {
            if maybe_last.is_empty_dir() {
                *maybe_last = Record::File(contents);
                true
            } else {
                false
            }
        }).unwrap_or(false)
    }

    fn list(&self) -> Option<Vec<PathBuf>> {
        match self {
            Record::File(_) => {
                None
            }
            Record::Dir(e) => {
                let files: Vec<_> = e.keys().map(|c| c.clone()).collect();
                Some(files)
            }
        }
    }
}

pub struct MockFS {
    root_path: PathBuf,
    root_dir: RwLock<Record>,
}

impl MockFS {
    pub fn new<T: Into<PathBuf>>(root_path: T) -> Self {
        MockFS {
            root_path: root_path.into(),
            root_dir: RwLock::new(Record::Dir(HashMap::default())),
        }
    }

    pub fn with_file<P: AsRef<Path>, B: Into<Vec<u8>>>(mut self, path: P, bytes: B) -> Self {
        self.add_file(path.as_ref(), bytes.into()).unwrap_or_else(
            |_| error!("failed creating file in mockfs"));
        self
    }

    pub fn with_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.add_dir(path.as_ref()).unwrap_or_else(
            |_| error!("failed creating dir in mockfs"));
        self
    }

    pub fn add_dir(&self, path: &Path) -> Result<(), ()> {
        if self.root_dir.blocking_write().create_dir(path) { Ok(()) } else { Err(()) }
    }

    pub fn add_file(&mut self, path: &Path, bytes: Vec<u8>) -> Result<(), ()> {
        if self.root_dir.blocking_write().create_file(path, bytes) { Ok(()) } else { Err(()) }
    }

    pub fn blocking_overwrite_with_bytes(&self, path: &Path, bytes: Vec<u8>) -> Result<usize, WriteError> {
        let comp: Vec<_> = path.components().collect();

        if let Some(record) = self.root_dir.blocking_write().get_mut(&comp, false) {
            if record.is_dir() {
                return Err(WriteError::NotAFile);
            }

            let len_bytes = bytes.len();
            *record = Record::File(bytes);
            Ok(len_bytes)
        } else {
            Err(WriteError::FileNotFound)
        }
    }
}

impl Debug for MockFS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockFilesystem({})", self.root_path.to_string_lossy())
    }
}

#[async_trait]
impl FilesystemFront for MockFS {
    fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    fn blocking_read_entire_file(&self, path: &Path) -> Result<Vec<u8>, ReadError> {
        let comp: Vec<_> = path.components().collect();
        if let Some(rec) = self.root_dir.blocking_read().get(&comp) {
            match rec {
                Record::File(contents) => Ok(contents.clone()),
                Record::Dir(_) => Err(ReadError::NotAFilePath)
            }
        } else {
            Err(ReadError::FileNotFound)
        }
    }

    fn is_dir(&self, path: &Path) -> bool {
        let comp: Vec<_> = path.components().collect();
        self.root_dir.blocking_read().get(&comp).map(|r| r.is_dir()).unwrap_or(false)
    }

    fn is_file(&self, path: &Path) -> bool {
        let comp: Vec<_> = path.components().collect();
        self.root_dir.blocking_read().get(&comp).map(|r| r.is_file()).unwrap_or(false)
    }

    fn hash_seed(&self) -> usize {
        2
    }

    fn blocking_list(&self, path: &Path) -> Result<Vec<DirEntry>, ListError> {
        if !self.exists(path) {
            return Err(ListError::PathNotFound);
        }

        if !self.is_dir(path) {
            return Err(ListError::NotADir);
        }

        let comp: Vec<_> = path.components().collect();
        let items = if comp.is_empty() {
            self.root_dir.blocking_read().list()
        } else {
            match self.root_dir.blocking_read().get(&comp) {
                None => {
                    error!("this test was redundant and still failed!");
                    return Err(ListError::PathNotFound);
                }
                Some(dir) => dir.list()
            }
        };

        match items {
            None => {
                error!("this test was redundant 2 and still failed!");
                Err(ListError::NotADir)
            }
            Some(mut items) => {
                items.sort();
                Ok(items.into_iter().map(|i| DirEntry::new(i)).collect())
            }
        }
    }

    fn exists(&self, path: &Path) -> bool {
        let comp: Vec<_> = path.components().collect();
        self.root_dir.blocking_read().get(&comp).is_some()
    }

    fn blocking_overwrite_with_stream(&self, path: &Path, stream: &mut dyn StreamingIterator<Item=[u8]>) -> Result<usize, WriteError> {
        let mut bytes = Vec::<u8>::new();
        while let Some(chunk) = stream.next() {
            for i in chunk.into_iter() {
                bytes.push(*i);
            }
        };

        self.blocking_overwrite_with_bytes(path, bytes)
    }

    fn blocking_overwrite_with_str(&self, path: &Path, s: &str) -> Result<usize, WriteError> {
        let bytes: Vec<_> = s.bytes().collect();
        self.blocking_overwrite_with_bytes(path, bytes)
    }

    async fn to_fsf(self) -> FsfRef {
        FsfRef::new(self).await
    }
}

// these are purely API tests, like "does it have semantics I like", not "does it work well"
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use crate::de;
    use crate::fs::filesystem_front::FilesystemFront;
    use crate::fs::mock_fs::{MockFS, Record};
    use crate::fs::read_error::ReadError;

    #[test]
    fn make_some_records() {
        let mut record = Record::Dir(HashMap::new());

        let some_path = PathBuf::from("hello/some/path/item.txt");
        let comps: Vec<_> = some_path.components().collect();

        assert!(record.get(&comps[0..1]).is_none());
    }

    #[test]
    fn make_some_files() {
        let mockfs = MockFS::new("/tmp")
            .with_file("folder1/file1.txt", "some text")
            .with_file("folder2/file2.txt", "some text2");


        assert_eq!(mockfs.is_dir(&Path::new("folder1")), true);
        assert_eq!(mockfs.is_dir(&Path::new("folder2")), true);
        assert_eq!(mockfs.is_dir(&Path::new("folder3")), false);
        assert_eq!(mockfs.is_dir(&Path::new("")), true);

        assert_eq!(mockfs.is_file(&Path::new("folder1/file1.txt")), true);
        assert_eq!(mockfs.is_file(&Path::new("folder2/file2.txt")), true);
        assert_eq!(mockfs.is_file(&Path::new("folder1")), false);
        assert_eq!(mockfs.is_file(&Path::new("folder2")), false);
        assert_eq!(mockfs.is_file(&Path::new("")), false);

        assert_eq!(mockfs.blocking_list(&Path::new("")).unwrap(), vec![de!("folder1"), de!("folder2")]);

        assert_eq!(mockfs.blocking_read_entire_file(&Path::new("")), Err(ReadError::NotAFilePath));
        assert_eq!(mockfs.blocking_read_entire_file(&Path::new("/folder3")), Err(ReadError::FileNotFound));
        assert_eq!(mockfs.blocking_read_entire_file(&Path::new("folder2")), Err(ReadError::NotAFilePath));
        assert_eq!(mockfs.blocking_read_entire_file(&Path::new("folder1/file1.txt")), Ok("some text".as_bytes().to_vec()));
        assert_eq!(mockfs.blocking_read_entire_file(&Path::new("folder1/file3.txt")), Err(ReadError::FileNotFound));
    }
}