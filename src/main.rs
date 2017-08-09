#![allow(non_snake_case)]
extern crate crossbeam;
extern crate bufstream;
extern crate native_tls;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod segfo;
use std::fs::File;
use bufstream::BufStream;
use std::io::Read;

use segfo::configure::Config::*;
use segfo::error::Error::NetworkListenerError;
use segfo::listener::Listener::*;

fn loadConfig()->Result<ServerConfig,NetworkListenerError>{
    let conf = ServerConfig::new();
    match conf.loadConfig(){
        Err(_)=>{
            println!("設定ファイルの読み込みに失敗したため、新しく作成します。");
            println!("古いファイルは保持されています。");
            conf.storeConfig()?;
            println!("設定ファイルを作成しました。");
            Ok(conf)
        },
        Ok(conf)=>Ok(conf)
    }
}

fn dataRecieved(stream:&mut StreamInterface)->Result<(),NetworkListenerError>{
    let mut buf=vec![0u8;1024*1024*4];
    let mut stream = BufStream::new(stream);
    let size = stream.read(&mut buf[..]).unwrap();
    for i in 0..size{
        print!("{0:>02x} ",buf[i]);
    }
    println!("");
    Ok(())
}

fn main() {
    let conf = match loadConfig(){
        Ok(conf)=>conf,
        Err(e)=>{
            print!("{}",e);
            return;
        }
    };
    let mut svr:NetworkListener = match NetworkListener::new("127.0.0.1:19999"){
        Ok(r)=>r,
        Err(e)=>{
            println!("{}",e);
            return;
        }
    };
    svr.setRequestHandler(dataRecieved);
    if conf.tlsEnable==true{
        svr.listenServerTLS(&conf.certificate.filePath,&conf.certificate.passphrase).unwrap_or_else(|e|print!("{}",e))
    }else{
        svr.listenServer()
    }
}
