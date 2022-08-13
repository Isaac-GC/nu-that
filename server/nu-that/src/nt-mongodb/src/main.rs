use mongodb::{Client, options::ClientOptions};

// TODO:
// - Create DB connection Handler
// - Create Insert Documents Function
// - Retrieve/Delete Documents Function
// - Search Documents Function


// This is hardcoded for now. This will eventually be in another module
async fn insert_documents() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://root:example@127.0.0.1:27017").await?;
    client_options.app_name = Some("nu_that".to_string());
    
    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
