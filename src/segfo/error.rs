pub mod Error{
    #[derive(Debug)]
    pub enum NetworkListenerError{
        TlsError(::native_tls::Error),
        ParserError(::serde_json::Error),
        IoError(::std::io::Error),
    }
    impl From<::native_tls::Error> for NetworkListenerError {
        fn from(err: ::native_tls::Error) -> NetworkListenerError {
            NetworkListenerError::TlsError(err)
        }
    }

    impl From<::serde_json::Error> for NetworkListenerError {
        fn from(err: ::serde_json::Error) -> NetworkListenerError {
            NetworkListenerError::ParserError(err)
        }
    }

    impl From<::std::io::Error> for NetworkListenerError {
        fn from(err: ::std::io::Error) -> NetworkListenerError {
            NetworkListenerError::IoError(err)
        }
    }

    impl ::std::fmt::Display for NetworkListenerError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NetworkListenerError::TlsError(ref err) => write!(f, "Tls Error: {}", err),
                NetworkListenerError::ParserError(ref err) => write!(f, "Parse Error: {}", err),
                NetworkListenerError::IoError(ref err) => write!(f, "IO Error: {}", err),
            }
        }
    }

    impl ::std::error::Error for NetworkListenerError {
        fn description(&self) -> &str {
            match *self {
                NetworkListenerError::TlsError(ref err) => err.description(),
                NetworkListenerError::ParserError(ref err) => err.description(),
                NetworkListenerError::IoError(ref err) => err.description(),
            }
        }

        fn cause(&self) -> Option<&::std::error::Error> {
            match *self {
                NetworkListenerError::TlsError(ref err) => Some(err),
                NetworkListenerError::ParserError(ref err) => Some(err),
                NetworkListenerError::IoError(ref err) => Some(err),
            }
        }
    }
}
