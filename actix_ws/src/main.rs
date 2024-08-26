use mongo_lib::mongo;
use actix_web::{get,HttpRequest, Result, web, App, HttpServer};

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let conn = "mongodb+srv://mdeb:Tristan2006@cluster0.ziyu4.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
    let uri = conn.to_string();
    println!("Mongo URI : {}", uri);
    let _repo = mongo::mongo::init_connection(uri).await;

    HttpServer::new(|| {
        App::new()
            .service(index)
            //.route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mongo;
    use crate::mongo::mongo::init_connection;
    
    #[actix_web::test]
    async fn db_connection() {
        let conn = "mongodb+srv://mdeb:Tristan2006@cluster0.ziyu4.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
        let uri = conn.to_string();
        println!("Mongo URI : {}", uri);
        let repo = mongo::mongo::init_connection(uri).await;
        assert_eq!(repo.is_some(), true);
    }
    
}

