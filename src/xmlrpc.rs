use std::io::TcpStream;
use regex::Regex;

pub struct Client {
    pub server_uri: String
}

#[deriving(Show, PartialEq)]
pub enum Value {
    Empty,
    Int (int),
    Boolean (bool),
    String (String),
    Double (f64),
    // Currently not handling dateTime.iso8601 base64, or struct types
}

#[deriving(Show, PartialEq)]
pub enum Response {
    Success {param: Value},
    Fault {fault_code: int, fault_string: String},
}

#[deriving(Show, PartialEq)]
pub struct Request {
    pub method_name: String,
    pub params: Vec<Value>,
}

impl Client {
    pub fn execute_request(&self, request: &Request) -> Result<Response, String> {

        let mut stream = TcpStream::connect(self.server_uri.as_slice()).unwrap();

        let message = create_http_post(serialize_request(request).as_slice());

        // Send request to server
        match stream.write(message.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }

        // Read response from server
        let response_str = match stream.read_to_string() {
            Ok(response_str) => response_str,
            Err(err) => panic!("{}", err),
        };

        // Parse response
        match deserialize_response(response_str.as_slice()) {
            Ok(response) => Ok(response),
            Err(err) => Err(err)
        }
    }
}

fn deserialize_response(response_str: &str) -> Result<Response, String> {
    let param_re = match Regex::new(r"<value><([a-z0-9]+)>([^<]*)") {
        Ok(re) => re,
        Err(err) => return Err(format!("Parse error: {}", err)),
    };

    let mut num_params = 0i;
    let mut param = Value::Empty;
    for cap in param_re.captures_iter(response_str) {
        param = Value::String(cap.at(1).to_string());
        num_params += 1;
    }

    // XMLRPC allows zero or one returned params in a response
    match num_params {
        0|1 => Ok(Response::Success {param: param}),
        _ => Err(format!("Too many parameters in response ({})", num_params)),
    }
}

fn create_http_post(body: &str) -> String {
    format!(
        "POST /RPC2 HTTP/1.0\n\
        User-Agent: RosRust/0.0\n\
        Host: localhost\n\
        Content-Type: text/xml\n\
        Content-length: {content_length}\n\n{body}", content_length=body.len(), body=body)
}

fn serialize_request(request: &Request) -> String {
    let mut param_str = "".to_string();
    for param in request.params.iter() {
        param_str = param_str + format!("<param><value><string>{}</string></value></param>", param);
    };

    format!(
    "<?xml version=\"1.0\"?>\n\
    <methodCall>\n\
    <methodName>{}</methodName>\n\
    <params>\n\
      <param>\n\
      {}\n\
      </param>\n\
    </params>\n\
    </methodCall>\n", request.method_name, param_str)
}

#[test]
fn test_deserialize_response_good() {
    let response_str =
    "<?xml version=\"1.0\"?>\n\
    <methodResponse>\n\
       <params>\n\
          <param>\n\
             <value><string>param1</string></value>\n\
          </param>\n\
       </params>\n\
    </methodResponse>\n";

    let response = match deserialize_response(response_str) {
        Ok(response) => response,
        Err(_) => return assert!(false),
    };
    let correct_response = Response::Success {param: Value::String("param1".to_string())};
    assert_eq!(response, correct_response);
}

#[test]
fn test_deserialize_response_too_many_params() {
    let response_str =
    "<?xml version=\"1.0\"?>\n\
    <methodResponse>\n\
       <params>\n\
          <param>\n\
             <value><string>param1</string></value>\n\
             <value><string>param2</string></value>\n\
          </param>\n\
       </params>\n\
    </methodResponse>\n";

    match deserialize_response(response_str) {
        Ok(_) => return assert!(false),
        Err(_) => return (),
    };
}

