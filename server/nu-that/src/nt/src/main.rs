use vuln_collector::extract_osv_json_data;
use futures::executor::block_on;
use serde_json::json;

#[tokio::main]
async fn main() {
    let json_data = block_on(extract_osv_json_data("PyPI"));

    match json_data {
        Ok(v) => {
            for i in v.iter() {
                println!("Data: {:?}", i);
            }
        },
        Err(e) => println!("error: {e:?}"),
    }
    
    // for i in json_data.iter() {
    //     println!("Data: {:?}", i);
    // }
}
