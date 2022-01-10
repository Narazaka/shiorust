mod common;
pub mod message;

#[cfg(test)]
mod tests {
    use super::*;
    use message::traits::*;
    use message::*;

    #[test]
    fn parse_request() {
        let request =
            Request::parse("GET SHIORI/3.0\r\nID: AAA\r\nCharset: UTF-8\r\nReference0: foo\\nbar\r\nX-Ray: y\r\n\r\n").unwrap();
        assert_eq!(request.method.to_string(), "GET");
        assert_eq!(request.version.to_string(), "3.0");
        println!("'{}'", request);
        assert_eq!(request.headers.get("ID").unwrap().as_str(), "AAA");
        assert_eq!(request.get_header("ID").unwrap().as_str(), "AAA");
        assert_eq!(request.get_header("Charset").unwrap().as_str(), "UTF-8");
        assert_eq!(
            request.get_header("Reference0").unwrap().as_str(),
            "foo\\nbar"
        );
        assert_eq!(request.get_header("Reference0"), request.get_reference(0));
        assert_eq!(request.headers.get("Reference0"), request.get_reference(0));
        assert_eq!(request.get_header("X-Ray").unwrap().as_str(), "y");
    }

    #[test]
    fn parse_response() {
        let request =
            Response::parse("SHIORI/3.0 200 OK\r\nID: AAA\r\nCharset: UTF-8\r\nReference0: foo\\nbar\r\nX-Ray: y\r\n\r\n").unwrap();
        assert_eq!(request.status.to_string(), "200 OK");
        assert_eq!(request.version.to_string(), "3.0");
        println!("'{}'", request);
        assert_eq!(request.get_header("ID").unwrap().as_str(), "AAA");
        assert_eq!(request.get_header("Charset").unwrap().as_str(), "UTF-8");
        assert_eq!(
            request.get_header("Reference0").unwrap().as_str(),
            "foo\\nbar"
        );
        assert_eq!(request.get_header("Reference0"), request.get_reference(0));
        assert_eq!(request.get_header("X-Ray").unwrap().as_str(), "y");
    }
}
