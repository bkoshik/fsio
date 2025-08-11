use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::file::FileMetadata;

impl FileMetadata {
    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly"))]
    pub fn birth_time(&self) -> SystemTime {
        return UNIX_EPOCH + Duration::new(self.stat.st_birthtime as u64, self.stat.st_birthtime_nsec as u32);
    }
    
    pub fn modified_time(&self) -> SystemTime {
        return UNIX_EPOCH + Duration::new(self.stat.st_mtime as u64, self.stat.st_mtime_nsec as u32);
    }
    
    pub fn accessed_time(&self) -> SystemTime {
        return UNIX_EPOCH + Duration::new(self.stat.st_atime as u64, self.stat.st_atime_nsec as u32);
    }
    
    pub fn changed_time(&self) -> SystemTime {
        return UNIX_EPOCH + Duration::new(self.stat.st_ctime as u64, self.stat.st_ctime_nsec as u32);
    }
}