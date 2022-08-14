use chrono::{DateTime, Utc};
use vuln_collector::{extract_osv_json_data, fetch_osv_vuln_list_zip};
use futures::executor::block_on;
use serde_json::json;

#[tokio::main]
async fn main() {

    // Before retrieving a value, set the local epoch value to check for future updates.
    let dt: i64 = Utc::now().timestamp();

    let json_bytes = fetch_osv_vuln_list_zip("PyPI").await;

    // match json_bytes {
    //     Ok(jb) => json_data = Some(extract_osv_json_data(jb).await),
    //     Err(e) => println!("Error: {e:?}"),
    //           _=> panic!("Something went wrong! That's all we know"),
    // }

    // // let json_data = block_on(extract_osv_json_data("PyPI"));

    // if json_data.is_some() {
    //     match json_data {
    //         Some(Ok(v)) => {
    //             for i in v.iter() {
    //                 println!("Data: {:?}", i);
    //             }
    //         },
    //         Some(Err(e)) => println!("error: {e:?}"),
    //     }
    // }
    
    
    // for i in json_data.iter() {
    //     println!("Data: {:?}", i);
    // }
}
