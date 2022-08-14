use std::io::Read;

use zip::ZipArchive;
use serde_json::from_str;
use crate::ovf_schema::OvfFormat;

pub mod ovf_schema;

type Error = Box<dyn std::error::Error>;

// const BASE_URL: &str = "https://osv-vulnerabilities.storage.googleapis.com";
const BUCKET_NAME: &str = "osv-vulnerabilities";
const BASE_API_URL: &str = "https://storage.googleapis.com/storage/v1/b/";

// ToDo
// - Write function to download zip file [Done]
// - Write function to check for updated files
// - Write function to check/Mirror NIST NVD database

// Function is/should only be called during the initialization of the application
//   subsequent calls could be noisy and high-bandwidth usage.
pub async fn fetch_osv_vuln_list_zip(package_type: &str) -> Result<Vec<u8>, Error> {
    let object_url_encoded_path = [package_type, "all.zip"].join("%2F");
    let url = [BASE_API_URL, BUCKET_NAME, "o", &object_url_encoded_path].join("/");
    let response = reqwest::get(url).await?.bytes().await?;
    let buffer = response.to_vec();

    Ok(buffer)
}


// Function should only be called in conjuction with the 'fetch_osv_vuln_list_zip' function.
pub async fn extract_osv_json_data(json_byte_data: Vec<u8>) -> Result<Vec<OvfFormat>, Error> {
    let reader = std::io::Cursor::new(&json_byte_data);
    let mut archive = ZipArchive::new(reader).unwrap();
    let mut json_vec = Vec::new();

    // Get the number of files and extract them in memory to json documents
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap(); 
        let mut jd_buffer = String::new();

        zip_file.read_to_string(&mut jd_buffer)?;
        let ovfdoc: OvfFormat = from_str(&jd_buffer)?;
        println!("{:?}", &ovfdoc);

        json_vec.push(ovfdoc);        
    }

    Ok(json_vec)
}



// Function should be used for subsequent calls. It will only update files if the modified date is newer than the
//   last_epoch_update value, or if there are new files with a created time newer than the last_epoch_update value
// pub async fn check_for_updated_files(package_type: &str, last_epoch_update: i64) -> Result<Vec<OvfFormat>, Error> {

//     let current_datetime: i64 = Utc::now().timestamp();

//     if current_datetime > last_epoch_update {

//     // First check to see if anything was modified
    

//     }

// }