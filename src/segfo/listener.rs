pub mod Listener{
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read,Write};
    use segfo::error::Error::NetworkListenerError;
    use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
    use std::sync::Arc;
    use std::fs::File;
//    use std::time::Duration;
    
    pub trait StreamInterface:Write+Read{}
    impl StreamInterface for TcpStream{}
    impl StreamInterface for TlsStream<TcpStream>{}

    pub struct NetworkListener{
        listener:TcpListener,
        dataRecieveHandler:fn(stream:&mut StreamInterface)->Result<(),NetworkListenerError>,
    }

    impl NetworkListener{
        pub fn new(binderAddress:&'static str)->Result<NetworkListener,NetworkListenerError>{
            match TcpListener::bind(binderAddress){
                Ok(r)=>Ok(
                    NetworkListener{
                            listener:r,
                            dataRecieveHandler:NetworkListener::dataRecieveHandler,
                        }
                    ),
                Err(e)=>Err(::std::convert::From::from(e)),
            }
        }

        fn dataRecieveHandler(_:&mut StreamInterface)->Result<(),NetworkListenerError>{
            unimplemented!()
        }

        pub fn setRequestHandler(&mut self,dataRecieveHandler:fn(reader:&mut StreamInterface)->Result<(),NetworkListenerError>){
            self.dataRecieveHandler = dataRecieveHandler;
        }

        fn protocolHandler(&self,stream:&mut StreamInterface)->Result<(),NetworkListenerError>{
            (self.dataRecieveHandler)(stream)
        }

        fn initTLS(certFile:&str,certPass:&str)->Result<Arc<TlsAcceptor>,NetworkListenerError>{
            let mut file = File::open(certFile)?;
            let mut pkcs12 = vec![];
            file.read_to_end(&mut pkcs12)?;
            let pkcs12 = Pkcs12::from_der(&pkcs12, certPass)?;
            let builder = TlsAcceptor::builder(pkcs12)?;
            let acceptor = builder.build()?;
            Ok(Arc::new(acceptor))
        }

        pub fn listenServerTLS(&self,certFile:&str,certPass:&str)->Result<(),NetworkListenerError>{
            let acceptor = ::NetworkListener::initTLS(certFile,certPass)?;
            ::crossbeam::scope(|scope| {
                for stream in self.listener.incoming(){
                    let acceptor = acceptor.clone();
                    let stream = match stream{
                        Ok(s)=>s,
                        Err(_)=>continue
                    };
                    let mut stream = match acceptor.accept(stream){
                        Ok(stream)=>stream,
                        Err(e)=>{
                            println!("{}",e);
                            continue;
                        }
                    };
                    let h = scope.spawn(move||{
                        match self.protocolHandler(&mut stream){
                            Ok(_)=>{},
                            Err(e)=>{
                                println!("{}",e);
                            }
                        }
                        println!("next stream.");
                    });
                }
            });
            Ok(())
        }
        pub fn listenServer(&self){
            ::crossbeam::scope(|scope| {
                for stream in self.listener.incoming(){
                    let mut stream = match stream{
                        Ok(s)=>s,
                        Err(_)=>continue
                    };
                    let h = scope.spawn(move||{
                        match self.protocolHandler(&mut stream){
                            Ok(_)=>{},
                            Err(e)=>{
                                println!("{:?}",e);
                            }
                        }
                        println!("next stream.");
                    });
                }
            });
        }
    }
}