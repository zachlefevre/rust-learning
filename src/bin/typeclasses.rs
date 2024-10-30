mod example {
    use self::protocol::Protocol;

    mod nfs {
        #[derive(Clone)]
        pub struct AuthInfo(pub String);
    }

    mod bootp {
        pub struct AuthInfo(pub i32);
    }

    mod protocol {
        use super::{nfs::{self, AuthInfo}, bootp};


        pub struct TransferParameters {
            chunk_size: u32,
            bitrate: u32,
        }

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
        pub struct Bootp;

        impl Protocol for Bootp {
            type AuthInfo = bootp::AuthInfo;

            fn auth_info(&self) -> Self::AuthInfo {
                bootp::AuthInfo(18)
            }

        }
    }


    struct FileDownloadRequest<P: protocol::Protocol> {
        file_name: String, protocol: P
    }

    impl FileDownloadRequest<protocol::Nfs> {
        fn mount_point(&self) -> &String {
            let mp = &self.protocol.mount_point;
            let ai = self.protocol.auth_info();
            let s = ai.0;
            mp
        }
    }

    impl FileDownloadRequest<protocol::Bootp> {
        fn other(&self) -> i32 {
            let ai = self.protocol.auth_info();
            let s = ai.0;
            s
        }
    }
}



fn main() {
    
}
