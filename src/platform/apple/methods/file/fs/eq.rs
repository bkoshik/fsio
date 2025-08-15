use crate::file::File;

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        return self.metadata().unwrap().inode() == other.metadata().unwrap().inode();
    }
}