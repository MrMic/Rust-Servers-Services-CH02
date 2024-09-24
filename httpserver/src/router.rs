use http::{
    httprequest::{self, HttpRequest},
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        // TODO: Implement Router
        match req.method() {
            // If GET request
            httprequest::Method::Get => match &req.resource {},
            // If method is not GET request, return 404 page
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
