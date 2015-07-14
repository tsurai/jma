#![allow(unused_must_use)]

extern crate hyper;
extern crate mecab;
extern crate url;

use std::io::{Read, Write};
use mecab::Tagger;
use hyper::{Get, Post};
use hyper::server::{Server, Request, Response};
use hyper::net::Fresh;
use hyper::header::ContentType;
use url::percent_encoding::percent_decode;

fn main() {
  let header = include_str!("html/header.html");
  let index = include_str!("html/index.html");
  let sidebar = include_str!("html/sidebar.html");

  Server::http("127.0.0.1:62011").unwrap().handle(move |mut req: Request, mut res: Response<Fresh>| {
    res.headers_mut().set(ContentType::html());

    let mut res = res.start().unwrap();
    res.write(header.as_bytes());

    match req.method {
      Get => {
        res.write(index.as_bytes());
      },
      Post => {
        let mut input = "".to_string();
        req.read_to_string(&mut input);

        if input.starts_with("i=") {
          let mut tagger = Tagger::new("");

          res.write(sidebar.as_bytes());

          for node in tagger.parse_to_node(percent_decode(&input[2..].as_bytes())).iter_next() {
            let feature: Vec<&str> = node.feature.split(',').collect();
            if feature[0] == "記号" && feature[6] == "*" {
              res.write(b"<br>");
            } else if feature[0] != "BOS/EOS" && feature[6] != "　" {
              res.write(format!("<span class=\"word\" data-pos=\"{}\" data-desc=\"{}\" data-extdesc=\"{}\" data-verbgroup=\"{}\" data-baseform=\"{}\" data-dictform=\"{}\" data-reading=\"{} | {}\">{}</span>", feature[0], feature[1], feature[2], feature[3], feature[4], feature[5], feature[6], if feature.len() > 7 && feature[6] != feature[7] { feature[7] } else { "*" }, &node.surface[..(node.length as usize)]).as_bytes());
            }
          }
        }
      },
      _ => {}
    }
    
    res.write(b"</body></html>");
    res.end().unwrap();
  });
}
