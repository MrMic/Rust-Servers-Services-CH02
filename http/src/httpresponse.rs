use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

// ╾────────────────────────────────────────────────────────────────────╼
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

// ╾────────────────────────────────────────────────────────────────────╼
impl<'a> HttpResponse<'a> {
    // ______________________________________________________________________
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    // ______________________________________________________________________
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
}

// ╾╼ * INFO: Getters Methods for the HttpResponse ╾──────────────────╼
impl<'a> HttpResponse<'a> {
    // ______________________________________________________________________
    fn version(&self) -> &str {
        self.version
    }

    // ______________________________________________________________________
    fn status_code(&self) -> &str {
        self.status_code
    }

    // ______________________________________________________________________
    fn status_text(&self) -> &str {
        self.status_text
    }

    // ______________________________________________________________________
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string = String::from("");
        for (k, v) in map.iter() {
            header_string = format!("{}{}: {}\r\n", header_string, k, v);
        }
        header_string
    }

    // ______________________________________________________________________
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

// * INFO: From Trait ╾────────────────────────────────────────────────────────────────
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

// *  TEST: ═════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
}
