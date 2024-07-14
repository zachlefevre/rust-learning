mod example {
    use self::protocol::Protocol;

    mod nfs {
        #[derive(Clone)]
        pub struct AuthInfo(String);
    }

    mod bootp {
        pub struct AuthInfo(i32);
    }

    mod protocol {
        use super::nfs::{self, AuthInfo};


        pub trait Protocol {
            type AuthInfo;
            fn auth_info(&self) -> Self::AuthInfo;
        }

        pub struct Nfs {
            auth: AuthInfo,
            pub mount_point: String,
        }

        impl Protocol for Nfs {
            type AuthInfo = nfs::AuthInfo;

            fn auth_info(&self) -> Self::AuthInfo {
                self.auth.clone()
            }
        }
        struct Bootp;


    }


    struct FileDownloadRequest<P: protocol::Protocol> {
        file_name: String, protocol: P
    }

    impl FileDownloadRequest<protocol::Nfs> {
        fn mount_point(&self) -> &String {
            let mp = &self.protocol.mount_point;
            mp
        }
    }
}



fn main() {
    
}
