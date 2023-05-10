pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
}

pub struct Response {
    pub status: u16,
    pub body: String,
}

pub trait WasmHttpEndpoint {
    fn handle_request(&self, request: Request) -> Response;
}
