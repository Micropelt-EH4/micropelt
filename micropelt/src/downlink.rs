#[macro_export]
macro_rules! DownlinkStatus {
    ( $port:ident, $get:expr ) => {
        #[derive(Debug)]
        pub struct DownlinkStatus {}

        impl Downlink {
            pub fn status() -> DownlinkStatus {
                DownlinkStatus {}
            }
        }

        impl fmt::Display for DownlinkStatus {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Get {}", $get)
            }
        }

        impl lorawan::Downlink for DownlinkStatus {
            fn serialise(&self) -> Result<PortPayload> {
                Ok(PortPayload {
                    port: Port::$port as u8,
                    payload: Vec::with_capacity(0),
                })
            }
        }
    };
}
