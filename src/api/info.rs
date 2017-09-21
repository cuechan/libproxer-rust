#![allow(dead_code)]
#![allow(warnings)]
#![allow(unused)]
#![warn(missing_docs)]


use ApiResponse;
use chrono;
use error;
use error::api;
use prelude::*;
use Proxer;
use request;
use request::info::*;
use reqwest;
use reqwest::IntoUrl;
use response::info;
use serde_derive;
use serde_json;
use serde_json::Value;
use std;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;
use std::thread;
use std::time;


pub struct Info<'a> {
    pub proxer: Proxer<'a>
}


impl<'a> Info<'a> {
    pub fn get_fullentry(self, eid: InfoID) -> Result<info::FullEntry, error::Error> {
        let mut request = request::Request::new("info/fullentry");

        request.set_parameter("id", eid);


        let res = self.proxer.execute(request);





        if res.is_err() {
            return Err(res.err().unwrap());
        }


        // JSON PARsING

        let json;

        match res.unwrap().json::<ApiResponse>() {
            Ok(r) => json = r,
            Err(e) => return Err(error::Error::Json)
        }


        // API ERROR CHECKING

        if json.error != 0 {
            return Err(error::Error::Api(error::api::Api::from(json)))
        }


        let fullentry = info::FullEntry::from(json.data.unwrap());

        Ok(fullentry)
    }





    pub fn get_comments(self, vars: request::info::GetComments) -> Result<Vec<info::Comment>, error::Error> {
        let mut request = request::Request::new("info/comments");

        request.set_parameter("id", vars.id);
        if vars.p.is_some() {
            request.set_parameter("p", vars.p.unwrap());
        }

        match vars.p {
            Some(i) => request.set_parameter("p", i),
            None => request.set_parameter("p", ::api::DEFAULT_PAGER_PAGE)
        }


        match vars.limit {
            Some(i) => request.set_parameter("limit", i),
            None => request.set_parameter("limit", ::api::DEFAULT_PAGER_LIMIT)
        }

        match vars.sort {
            Some(i) => request.set_parameter("sort", i),
            None => request.remove_parameter("sort")
        }



        let res = self.proxer.execute(request);


        if res.is_err() {
            return Err(res.err().unwrap());
        }


        // JSON PARsING

        let api_res = match res.unwrap().json::<ApiResponse>() {
            Ok(r) => r,
            Err(e) => return Err(error::Error::Json)
        };


        // API ERROR CHECKING

        if api_res.error != 0 {
            return Err(error::Error::Api(error::api::Api::from(api_res)))
        }


        let data = match api_res.data.unwrap().as_array() {
            None => return Err(error::Error::Unknown),
            Some(x) => x.clone(),
        };


        let mut all_comments = Vec::new();
        for com in data {
            all_comments.push(info::Comment::from(com));
        }

        Ok(all_comments)
    }
}
