// This is using TypeState to enhance a Builder to ensure correct usage at compile time
// example pulled from https://github.com/jeremychone-channel/rust-builder/blob/main/src/web_states.rs
//
// Can do the same polymorphic behaviors as regular Builder pattern as desired

use std::marker::PhantomData;

// This struct represents a simple http request. This is the target this builder will create for us
// pub properties just so I can check them in the test to quiet the warnings
#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

// These are states for the builder to transition between.
// structs are required rather than an enum because these need to be different types.

#[derive(Default, Clone)]
pub struct Sealed;
#[derive(Default, Clone)]
pub struct NotSealed;

#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;
#[derive(Default, Clone)]
pub struct Method(String);

// Builder
#[derive(Debug, Default, Clone)]
pub struct RequestBuilder<UrlState, MethodState, SealedState> {
    url: UrlState,
    method: MethodState,
    headers: Vec<(String, String)>,
    body: Option<String>,
    marker_seal: PhantomData<SealedState>,
}

// This impl only exists on the Builder in the state with all no states.
impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn seal(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }
}

// Can build as long as Url and Method are defined, but whether sealed is applied or not doesn't matter
impl<S> RequestBuilder<Url, Method, S> {
    // could return Result or a dyn or whatever depending on if runtime checks or runtime polymorphism is needed
    // a more typical Builder might need Result to check that url is provided for example
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        }
    }
}

// these are available in any state other than the sealed state
impl<U, M> RequestBuilder<U, M, NotSealed> {
    // sets the Url state on and applies url
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: self.marker_seal,
        }
    }

    // sets the method and turn on Method state
    // probably a reasonable place to impeove by limiting method to an enum of what the supported ones are
    pub fn method(self, method: impl Into<String>) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
            marker_seal: self.marker_seal,
        }
    }

    // sets the body
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    // appends to the header
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
