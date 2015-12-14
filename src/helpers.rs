use ::hyper::client::Client;
use ::hyper::client::response::Response;
use ::hyper::header::Connection;

#[derive(Debug,Clone)]
pub struct FormSite {
    pub api_key: String,
    pub base_url: String,
}

impl FormSite {
    pub fn new(api_key: String,
               fs_x: usize,
               account: String,
               form: Option<String>,
               transaction: Option<String>) -> FormSite {
        let form = match form {
            None => "".to_string(),
            Some(s) => "/".to_string() + &s,
        };
        let transaction = match transaction {
            None => "".to_string(),
            Some(s) => "/".to_string() + &s,
        };
        let url_bits: Vec<String> = vec!["https://fs".to_string(),
                                         fs_x.to_string(),
                                         ".formsite.com/api/users/".to_string(),
                                         account,
                                         "/forms".to_string(),
                                         form,
                                         transaction];
        FormSite {
            api_key: api_key,
            base_url: url_bits.iter().fold(String::new(), |r, s| r + s),
        }
    }

    pub fn get(&self) -> Response {
        let param = "?fs_api_key=".to_string() + &self.api_key;
        let url = self.base_url.to_string() + &param;
        let client = Client::new();
        let request = client.get(&url).header(Connection::close());
        match request.send() {
            Err(e) => panic!("Error sending GET request: {}", e),
            Ok(res) => res,
        }
    }
}
