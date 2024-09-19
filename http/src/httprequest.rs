use std::collections::HashMap;

// ______________________________________________________________________
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

// ______________________________________________________________________
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

// ______________________________________________________________________
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

// ______________________________________________________________________
#[derive(Debug)]
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
        let mut parsed_msg_body = "";

        // Read each line in the incoming HTTP HttpRequest
        for line in req.lines() {
            // If the line read is request line, call function process_request_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
                // if line read is a header line, call function process_header_line()
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
                // if it is a blank line, do nothing
            } else if line.len() == 0 {
                // If none of these, treat it as message body
            } else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP Request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_request_line(s: &str) -> (Method, Resource, Version) {
    // Parse the request line into individual chunks split by whitespaces.
    let mut words = s.split_whitespace();
    // Extract the HTTP method from first part of the request line
    let method = words.next().unwrap();
    // Extract the resource (URI/URL) from second part of the request line
    let resource = words.next().unwrap();
    // Extract the HTTP version from third part of the request line
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line() {}

// * TEST: ═══════════════════════════════════════════════════════════
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
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
}
