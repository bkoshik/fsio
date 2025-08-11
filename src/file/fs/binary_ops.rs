use crate::file::File;

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        let self_inode = match self.metadata() {
            Ok(md) => md.inode(),
            Err(_) => return false
        };
        let other_inode = match other.metadata() {
            Ok(md) => md.inode(),
            Err(_) => return false
        };

        return self_inode == other_inode;
    }
}