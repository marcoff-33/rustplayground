use std::collections::HashMap;
// parser for  raw http request strings
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::new();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
                // do nothing
            } else {
                parsed_msg_body.push_str(line);
                parsed_msg_body.push('\n');
            }
        }

        let http_request = HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        };

        println!("Parsed HttpRequest: {:?}", http_request); // Debug print

        http_request
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    // parse the header line into words split by separator (':')
    let mut header_items = s.split(":");

    let mut key = String::from("");
    let mut value = String::from("");
    // the key part of the header
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // the value part of the header
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    println!("Parsed header: key = '{}', value = '{}'", key, value);
    (key, value)
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /catsvideo HTTP/1.1\r\nHost: youtube.com\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".to_string(), " youtube.com".to_string());
        headers_expected.insert("Accept".to_string(), " */*".to_string());
        headers_expected.insert("User-Agent".to_string(), " curl/7.64.1".to_string());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/catsvideo".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
