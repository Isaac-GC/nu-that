use mongodb::{Client, options::ClientOptions};

// TODO:
// - Create DB connection Handler
// - Create Insert Documents Function
// - Retrieve/Delete Documents Function
// - Search Documents Function


struct MongoDBConnector;

const BLANK_CONNECTION_ID = 0;

pub type ConnectionId = u16;

impl MongoDBConnector<T> {
    
    // ToDo: This needs to be improved to ensure the password remains secure in memory
    pub fn new(username: String, password: String, url: String, port: u16, conn_id: ConnectionId) -> Session<T> {
        assert_ne!(conn_id, BLANK_CONNECTION_ID);
        Self::new_internal(username, password, url, port, conn_id)
    }


    fn new_internal(username: String, password: String, url: String, port: u16, conn_id: u16) -> Session<T> {
        Session {
            username,
            password,
            url,
            port,
            conn_id,
        }
    }

    pub fn conn_id(&self) -> ConnectionId {
        self.conn_id
    }

    
}

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
