use actix_web::{
  guard, http::ContentEncoding, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use super::core::TokioRuntime;
use super::sys_schema;

async fn playground_html_handler() -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
      )),
  )
}

async fn playground_schema_handler(
  schema: web::Data<sys_schema::SysSchema>,
  req: Request,
) -> Response {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_handler(
  schema: web::Data<sys_schema::SysSchema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let rt = TokioRuntime::new();

  let sys_schema = Schema::build(
    sys_schema::RootQuery::default(),
    EmptyMutation,
    EmptySubscription,
  )
  .data(rt.handle())
  .finish();

  println!("System Playground: http://localhost:8000");

  HttpServer::new(move || {
    App::new()
      .data(sys_schema.clone())
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
  .bind("0.0.0.0:8000")?
  .run()
  .await
}
