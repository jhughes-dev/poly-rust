pub trait ResourceInterface {
    fn request(&self, url: &str) -> Option<String>;
}

pub struct ProxyResource {
    resource: Option<Box<dyn ResourceInterface>>,
}

impl ProxyResource {
    // For this example, we can construct the
    // proxy however we want, but in a production case,
    // you might not be explicit about returning a proxy,
    // and instead give access through a get_resource() function
    pub fn new() -> ProxyResource {
        // This function could return immediately, while something starts up in the background.
        ProxyResource { resource: None }
    }

    // This might be private in a real implementation, but it's useful for writing the tests
    pub fn set_resource(&mut self, resource: Box<dyn ResourceInterface>) {
        self.resource = Some(resource);
    }
}

impl ResourceInterface for ProxyResource {
    fn request(&self, url: &str) -> Option<String> {
        // Check the URL and decide whether to forward the request to the real resource
        if url.contains("yahoo") {
            return Some(format!("Blocked: {}", url));
        }

        // Some other examples of things to do here would be:
        // - Logging the request
        // - Caching the result
        // - Rate limiting the requests
        // - Setting up credentials
        // - Forwarding to a different resource (i.e. proxy server)
        if let Some(res) = &self.resource {
            return res.request(url);
        }
        // Could also do some post processing on the result
        None
    }
}
