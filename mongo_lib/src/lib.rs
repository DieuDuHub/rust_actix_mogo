#![allow(dead_code)]
pub mod mongo;
pub mod models;
#[macro_use]
pub mod error;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mongo;
    use crate::mongo::mongo::init_connection;

    #[tokio::test]
    async fn db_connection() {
        let conn = "mongodb+srv://mdeb:Tristan2006@cluster0.ziyu4.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
        let uri = conn.to_string();
        println!("Mongo URI : {}", uri);
        let repo = init_connection(uri).await;
        assert_eq!(repo.is_some(), true);
    }
    
}
