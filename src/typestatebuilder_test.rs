use crate::typestatebuilder::*;

#[test]
fn requester() {
    // doesn't require template args because new is only defined on the <no, no, no> version
    let req_builder = RequestBuilder::new()
        .url("https://www.some-url.com")
        .body("some body")
        .method("GET")
        .header("token name", "token value");

    let req = req_builder.build();

    assert_eq!(req.url, "https://www.some-url.com");
    assert_eq!(req.body, Some("some body".into()));
    assert_eq!(req.method, ("GET"));
    assert_eq!(
        req.headers,
        vec!(("token name".into(), "token value".into()))
    );

    let _req_builder = RequestBuilder::new();
    // fails to compile because required properties don't exist
    // let req = _req_builder.build();

    let _req_builder = RequestBuilder::new()
        .body("some body")
        .method("GET")
        .header("token name", "token value");

    // fails to compile because no url
    // let req = _req_builder.build();

    let _req_builder = RequestBuilder::new()
        .url("https://www.some-url.com")
        .body("some body")
        .header("token name", "token value");

    // fails to compile because no method
    // let req = _req_builder.build();

    let _req_builder = RequestBuilder::new()
        .url("https://www.some-url.com")
        .body("some body")
        .method("GET")
        .header("token name", "token value");

    let _req_builder = _req_builder.seal();

    // fails to compile because sealed
    // let req_builder = _req_builder.url("https://www.some-url.com");

    let _req = _req_builder.build();
}
