// This requires mount_point to work over FileDownloadRequests over Bootp, even though that is *nonsense*
mod bad {
    // these are protocol handlers
    pub mod nfs {
        pub struct AuthInfo {
        }
    }

    // these are protocol handlers
    pub mod bootp {
        pub struct AuthInfo {
        }
    }

    // this unifies the two protocol handlers to form the protocol interface
    enum AuthInfo {
        Nfs{mount_point: String, auth_info: nfs::AuthInfo}, Bootp{auth_info: bootp::AuthInfo}
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

mod good {
    pub mod nfs {
        #[derive(Clone)]
        pub struct AuthInfo(String);
    }

    pub mod bootp {
        pub struct AuthInfo();
    }

    pub mod protocol {
        use super::{nfs, bootp};

        pub trait ProtocolKind {
            type AuthInfo;
            fn auth_info(&self) -> Self::AuthInfo;
        }

        pub struct Nfs {
            auth: nfs::AuthInfo,
            pub mount_point: String
        }

        impl ProtocolKind for Nfs {
            type AuthInfo = nfs::AuthInfo;

            fn auth_info(&self) -> Self::AuthInfo {
                self.auth.clone()
            }
        }

        pub struct Bootp;

        impl ProtocolKind for Bootp {
            type AuthInfo = bootp::AuthInfo;

            fn auth_info(&self) -> Self::AuthInfo {
                bootp::AuthInfo()
            }
        }

    }

    pub struct FileDownloadRequest<P: protocol::ProtocolKind> {
        file_name: String, protocol: P
    }

    impl<P: protocol::ProtocolKind> FileDownloadRequest<P> {
        pub fn file_name(&self) -> &String {
            &self.file_name
        }
    }

    impl FileDownloadRequest<protocol::Nfs> {
        pub fn mount_point(&self) -> &String {
            &self.protocol.mount_point
        }
    }
}

fn main() {
    let req: crate::good::FileDownloadRequest<crate::good::protocol::Nfs> = todo!();
    req.file_name();
    req.mount_point();
    let req: crate::good::FileDownloadRequest<crate::good::protocol::Bootp> = todo!();
    req.file_name();
    //req.mount_point();

}
