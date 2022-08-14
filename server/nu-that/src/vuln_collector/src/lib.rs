use std::io::Read;

use zip::ZipArchive;
use serde_json::from_str;
use crate::ovf_schema::OvfFormat;

mod ovf_schema;

type Error = Box<dyn std::error::Error>;

async fn fetch_osv_vuln_list(package_type: &str) -> Result<Vec<u8>, Error> {
    let url = ["https://osv-vulnerabilities.storage.googleapis.com", package_type, "all.zip"].join("/");
    let response = reqwest::get(url).await?.bytes().await?;

    let buffer = response.to_vec();

    Ok(buffer)
}

pub async fn extract_osv_json_data(package_type: &str) -> Result<Vec<OvfFormat>, Error> {
    let json_byte_data = fetch_osv_vuln_list(package_type).await?;
    let reader = std::io::Cursor::new(&json_byte_data);
    let mut archive = ZipArchive::new(reader).unwrap();
    let mut json_vec = Vec::new();


    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap(); 
        let mut jd_buffer = String::new();

        zip_file.read_to_string(&mut jd_buffer)?;
        let mut ovfdoc: OvfFormat = from_str(&jd_buffer)?;

        // println!("{:?}", &ovfdoc);
        json_vec.push(ovfdoc);
        
    }

    Ok(json_vec)
}