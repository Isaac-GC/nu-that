use vuln_collector::extract_osv_json_data;
use futures::executor::block_on;
use serde_json::json;

#[tokio::main]
async fn main() {
    let json_data = block_on(extract_osv_json_data("PyPI"));

    // println!("{:?}", json!(json_data.iter().last()));
}
