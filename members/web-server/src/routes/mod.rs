use self::graphql::{MutationRoot, QueryRoot, TweetterSchema};
use actix_web::{
    get, post,
    web::{self as Web, Data},
    HttpResponse, Result, Scope,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod graphql;

#[post("/")]
async fn handler_graphql(
    schema: Web::Data<TweetterSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[get("/")]
async fn handler_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8080/graphql/")
                .finish(),
        ))
}

pub fn declare() -> Scope {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    Web::scope("/graphql")
        .app_data(Data::new(schema))
        .service(handler_graphql)
        .service(handler_graphiql)
}
