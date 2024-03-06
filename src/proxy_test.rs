use crate::proxy::*;

pub struct RealResource1;
pub struct RealResource2;
pub struct RealResource3;

impl ResourceInterface for RealResource1 {
    fn request(&self, url: &str) -> Option<String> {
        Some(format!("RealResource1: fetching from {}", url))
    }
}

impl ResourceInterface for RealResource2 {
    fn request(&self, url: &str) -> Option<String> {
        Some(format!("RealResource2: fetching from {}", url))
    }
}

impl ResourceInterface for RealResource3 {
    fn request(&self, url: &str) -> Option<String> {
        Some(format!("RealResource3: fetching from {}", url))
    }
}

#[test]
fn test_proxy() {
    let mut proxy = ProxyResource::new();
    // Nothing is setup yet
    assert!(proxy.request("http://www.google.com").is_none());

    proxy.set_resource(Box::new(RealResource1));

    assert_eq!(
        proxy.request("http://www.google.com").unwrap(),
        "RealResource1: fetching from http://www.google.com"
    );

    proxy.set_resource(Box::new(RealResource2));
    assert_eq!(
        proxy.request("http://www.google.com").unwrap(),
        "RealResource2: fetching from http://www.google.com"
    );

    proxy.set_resource(Box::new(RealResource3));
    assert_eq!(
        proxy.request("http://www.google.com").unwrap(),
        "RealResource3: fetching from http://www.google.com"
    );

    assert_eq!(
        proxy.request("http://www.yahoo.com").unwrap(),
        "Blocked: http://www.yahoo.com"
    );
}
