extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate select;

use futures::{future, Future, Stream};
use hyper::Client;
use hyper::header::Location;
use tokio_core::reactor::Core;
use select::document::{Document};
use select::predicate::{Class};

fn main() {
    let mut core = Core::new().expect("Core initialization failed");
    let client = Client::new(&core.handle());

    let uri = "http://bash.org.pl/random/".parse().expect("Parse failed");
    let work = client
        .get(uri)
        .and_then(|res| {
            future::ok(res.headers().get::<Location>().unwrap().clone())
        })
        .and_then(|loc| {
            client.get(loc.parse().expect("Parse failed"))
                .and_then(|res| {
                    res.body()
                        .fold(vec![], |mut v, chunk| {
                            v.extend(&chunk[..]);
                            future::ok::<_, hyper::Error>(v)
                        })
                        .and_then(|vec| {
                            let body = String::from_utf8(vec).expect("Formatting failed");
                            let doc = Document::from(&*body);
                            for node in doc.find(Class("post-body")) {
                                println!("{}", node.text());
                            }
                            future::ok(())
                        })                        
                })
        });
    core.run(work).expect("Start failed");
}
