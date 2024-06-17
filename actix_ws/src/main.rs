use mongo_lib::mongo;

#[actix_rt::main]
async fn main() {
    println!("Hello, world!");

    let conn = "mongodb+srv://mdeb:Tristan2006@cluster0.ziyu4.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
    let uri = conn.to_string();
    println!("Mongo URI : {}", uri);
    let _repo = mongo::mongo::init_connection(uri).await;

}

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
        let repo = mongo::mongo::init_connection(uri).await;
        assert_eq!(repo.is_some(), true);
    }
    
}

