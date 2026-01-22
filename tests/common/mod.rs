pub fn init_rewired_router(client: reqwest_rewire::Client) -> anyhow::Result<Router> {
    let client = Arc::new(reqwest_rewire::Client::ReqwestClient(reqwest_client));
    let state = ApiState { client };
    let router = Router::new()
        .route("/", get(great))
        .nest("/lines", crate::endpoints::lines::router())
        .nest("/stops", crate::endpoints::stops::router())
        .nest("/arrivals", endpoints::arrivals::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    Ok(router.with_state(state))
}
