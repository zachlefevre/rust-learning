// This requires mount_point to work over FileDownloadRequests over Bootp, even though that is *nonsense*
mod bad {
    pub mod nfs {
        pub struct AuthInfo {
        }
    }

    pub mod bootp {
        pub struct AuthInfo {
        }
    }

    enum AuthInfo {
        Nfs{auth_info: crate::bad::nfs::AuthInfo, mount_point: String}, Bootp{auth_info: crate::bad::bootp::AuthInfo}
    }

    struct FileDownloadRequest {
        file_name: String,
        auth_info: AuthInfo
    }

    impl FileDownloadRequest {
        fn mount_point(&self) -> Option<&String> {
            match &self.auth_info {
                AuthInfo::Nfs{mount_point, ..} => Some(mount_point),
                AuthInfo::Bootp{..} => None,
            }
        }
    }
}

fn main() {
    
}
