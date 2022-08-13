use zip::ZipArchive;
use serde_json::{from_reader, Value, json};
// use crate::ovf_schema::OvfFormat;

// mod ovf_schema;

type Error = Box<dyn std::error::Error>;

async fn fetch_osv_vuln_list(package_type: &str) -> Result<Vec<u8>, Error> {
    let url = ["https://osv-vulnerabilities.storage.googleapis.com", package_type, "all.zip"].join("/");
    let response = reqwest::get(url).await?.bytes().await?;

    let buffer = response.to_vec();

    Ok(buffer)
}

pub async fn extract_osv_json_data(package_type: &str) -> Result<Vec<Value>, Error> {
    let json_byte_data = fetch_osv_vuln_list(package_type).await?;
    let reader = std::io::Cursor::new(&json_byte_data);
    let mut archive = ZipArchive::new(reader).unwrap();
    let mut json_vec = Vec::new();
    
    let mut a = 0;

    for i in 0..archive.len() {
        let mut json_data = archive.by_index(i).unwrap();
        json_vec.push(from_reader(&mut json_data)?);
        // println!("{:?}",from_reader(&mut json_data)?);

        if a <= 1 {
            a += 1 ; 
            println!("{:?}", json!(from_reader(&mut json_data)?));
        }

        // OLD -- Mark for deletion
        // This will probably be never used as the vuln data from
        //    the osv-vulnerabilities has a flat data structure
        // let outpath = match file.enclosed_name() {
        //     Some(path) => path.to_owned(),
        //     None => continue,
        // };        
        // if (*file.name()).ends_with('/') {
        //     fs::create_dir_all(&outpath).unwrap();
        // }

        // if let Some(p) = outpath.parent() {
        //     if !p.exists() {
        //         fs::create_dir_all(&p).unwrap();
        //     }
        //     let mut outfile = fs::File::create(&outpath).unwrap();
        //     io::copy(&mut file, &mut outfile).unwrap();
        // }

    }
    Ok(json_vec)
}

// async fn fetch_url(url: String, file_name: String) -> Result<()> {
//     let response = reqwest::get(url).await?;
//     let mut file = std::fs::File::create(file_name)?;
//     let mut content =  Cursor::new(response.bytes().await?);
//     std::io::copy(&mut content, &mut file)?;
//     Ok(())
// }