use futures::lock::Mutex;
use std::sync::Arc;

use actix_web::{
  guard, http::ContentEncoding, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_actors::ws;

use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use super::ros2_schema::*;
use ros2_sys::Ros2Manager;

async fn playground_html_handler() -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
      )),
  )
}

async fn playground_schema_handler(schema: web::Data<Ros2Schema>, req: Request) -> Response {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_handler(
  schema: web::Data<Ros2Schema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let ros2 = Arc::new(Mutex::new(Ros2Manager::new()));
  {
    let mut guard = ros2.lock().await;
    guard.start_spinner();
    guard.create_node("leogated");
  }

  let schema = Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription)
    .data(ros2.clone())
    .finish();

  println!("ROS2 Playground: http://localhost:8002");

  HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .data(ros2.clone())
      .wrap(middleware::Compress::new(ContentEncoding::Auto))
      .service(
        web::resource("/")
          .guard(guard::Get())
          .guard(guard::Header("upgrade", "websocket"))
          .to(graphql_handler),
      )
      .service(
        web::resource("/")
          .guard(guard::Post())
          .to(playground_schema_handler),
      )
      .service(
        web::resource("/")
          .guard(guard::Get())
          .to(playground_html_handler),
      )
  })
  .bind("0.0.0.0:8002")?
  .run()
  .await
}
