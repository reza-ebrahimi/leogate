use actix_web::{
  guard, http::ContentEncoding, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_actors::ws;

use futures::lock::Mutex;
use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};

use ros_schema::*;
use ros_sys::RosManager;

mod binary_websocket_handler;
mod client_request;

use binary_websocket_handler::BinaryWebsocketHandler;

async fn graphql_post_handle(schema: web::Data<RosSchema>, req: Request) -> Response {
  schema.execute(req.into_inner()).await.into()
}

async fn binary_websocket_handler(
  ros: web::Data<Arc<futures::lock::Mutex<RosManager>>>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
  ws::start(
    BinaryWebsocketHandler::new(ros.get_ref().clone()),
    &req,
    payload,
  )
}

async fn graphql_handler(
  schema: web::Data<RosSchema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

async fn playground_handler() -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
      )),
  )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let ros = Arc::new(Mutex::new(ros_sys::RosManager::new()));
  ros.lock().await.create_node("leogated");

  let schema = Schema::build(
    RootQuery::default(),
    RootMutation::default(),
    RootSubscription::default(),
  )
  .data(ros.clone())
  .finish();

  println!("Playground: http://localhost:8000");

  HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .data(ros.clone())
      .wrap(middleware::Compress::new(ContentEncoding::Auto))
      .service(
        web::resource("/ws")
          .guard(guard::Get())
          .guard(guard::Header("upgrade", "websocket"))
          .to(binary_websocket_handler),
      )
      .service(
        web::resource("/")
          .guard(guard::Get())
          .guard(guard::Header("upgrade", "websocket"))
          .to(graphql_handler),
      )
      .service(
        web::resource("/")
          .guard(guard::Get())
          .to(playground_handler),
      )
      .service(
        web::resource("/")
          .guard(guard::Post())
          .to(graphql_post_handle),
      )
  })
  .bind("0.0.0.0:8000")?
  .run()
  .await
}
