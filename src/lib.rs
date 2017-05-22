
pub mod repository {

    extern crate hyper;
    extern crate hyper_native_tls;
    
    use std;

    #[derive(Debug)]
    pub enum Error {
        DownloadError(hyper::Error),
        ServerError(hyper::status::StatusCode),
        InvalidFileError,
        Success,
    }

    pub fn tap (url: &str) -> Error {

        // Download using Hyper
        println!("spam: Downloading repository file...");
        let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);
        let response = match client.get(url).send() { 
            Ok(v) => v, 
            Err(e) => return Error::DownloadError(e)
        };

        if response.status != hyper::status::StatusCode::Ok {
            return Error::ServerError(response.status);
        }

        // Parse and save to disk
        println!("spam: Parsing repository file...");
        let mut response_text = String::new();

        return Error::Success;
    }

    pub fn untap (name: &str) {
        
    }
}