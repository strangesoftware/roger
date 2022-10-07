use jsonrpc_v2::{Data, Error, Params, Server};

#[derive(serde::Deserialize)]
struct TwoNumbs {
    a: usize,
    b: usize,
}

async fn add(Params(params): Params<TwoNumbs>) -> Result<usize, Error> {
    Ok(params.a + params.b)
}

async fn sub(Params(params): Params<(usize, usize)>) -> Result<usize, Error> {
    Ok(params.0 - params.1)
}

async fn message(data: Data<String>) -> Result<String, Error> {
    Ok(String::from(&*data))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let rpc = Server::new()
        .with_data(Data::new(String::from("hey lol")))
        .with_method("sub", sub)
        .with_method("add", add)
        .with_method("message", message)
        .finish();

    actix_web::HttpServer::new(move || {
        let rpc = rpc.clone();
        actix_web::App::new().service(
            actix_web::web::service("/api")
                .guard(actix_web::guard::Post())
                .finish(rpc.into_web_service()),
        )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
