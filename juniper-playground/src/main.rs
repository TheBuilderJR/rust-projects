use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult,
    GraphQLEnum, Variables, graphql_value
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn favoriteEpisode(context: &Ctx) -> FieldResult<Episode> {
        Ok(context.0)
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn main() {
    let ctx = Ctx(Episode::NewHope);

    let (res, _errors) = juniper::execute_sync(
        "query { favoriteEpisode }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();

    assert_eq!(res, graphql_value!({ "favoriteEpisode": "NEW_HOPE" }));
}
