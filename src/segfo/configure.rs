pub mod Config{
    use std::io::{Write,Read,BufReader,BufWriter};
    use segfo::error::Error::NetworkListenerError;
    #[derive(Serialize, Deserialize)]
    pub struct ServerConfig {
        pub interface: String,
        pub port: String,
        pub tlsEnable:bool,
        pub certificate:Certificate
    }
    #[derive(Serialize, Deserialize)]
    pub struct Certificate{
        pub filePath:String,
        pub passphrase:String,
    }
    use std::result;
    type Result<T> = result::Result<T, NetworkListenerError>;
    impl ServerConfig{
        pub fn new()->ServerConfig {
            ServerConfig {
                interface: "::0".to_owned(),
                port: "443".to_owned(),
                tlsEnable:true,
                certificate: Certificate{
                    filePath:"identity.p12".to_owned(),
                    passphrase:"password".to_owned()
                }
            }
        }

        pub fn storeConfig(&self) -> Result<()> {
            // jsonファイルを保存
            let json = ::serde_json::to_string(&self)?;
            let mut writer = BufWriter::new(::File::create("serverconfig.json")?);
            let json = json.into_bytes();
            writer.write_all(& json)?;
            Ok(())
        }

        pub fn loadConfig(&self)-> Result<ServerConfig>{
            let mut reader = BufReader::new(::File::open("serverconfig.json")?);
            let mut json = String::new();
            reader.read_to_string(&mut json)?;

            let config: ServerConfig = match ::serde_json::from_str(&json){
                Ok(config)=>config,
                Err(e)=>{
                    let mut writer = BufWriter::new(::File::create("serverconfig.json.old")?);
                    let oldJson=json.to_string();
                    writer.write_all(&oldJson.into_bytes())?;
                    return Err(::std::convert::From::from(e));
                }
            };
            Ok(config)
        }
    }
}
