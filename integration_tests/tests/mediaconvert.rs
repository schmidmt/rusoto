#![cfg(feature = "mediaconvert")]

extern crate rusoto_core;
extern crate rusoto_mediaconvert;

use rusoto_mediaconvert::{MediaConvert, MediaConvertClient, ListJobsRequest};
use rusoto_core::Region;

#[test]
fn should_list_jobs() {
    let client = MediaConvertClient::new(Region::UsEast1);
    let request = ListJobsRequest::default();

    match client.list_jobs(request).sync() {
        Ok(resp) => println!("Got success response of {:?}", resp),
        Err(err) => assert!(format!("{}", err).contains("You must use the subscription API")), // needs to be enabled per account
    }
}
