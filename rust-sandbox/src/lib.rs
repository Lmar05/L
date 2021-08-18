use serde::{Deserialize, Serialize};
use worker::*;

mod counter;
mod test;
mod utils;
#[derive(Deserialize, Serialize)]
struct MyData {
    message: String,
    #[serde(default)]
    is: bool,
    #[serde(default)]
    data: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiData {
    user_id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct User {
    id: String,
    timestamp: u64,
    date_from_int: String,
    date_from_str: String,
}

fn handle_a_request(_req: Request, _env: Env, _params: Params) -> Result<Response> {
    Response::ok("hello, world.")
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let mut router = Router::new();

    router.get("/request", handle_a_request)?;
    router.post("/headers", |req, _, _| {
        let mut headers: http::HeaderMap = req.headers().into();
        headers.append("Hello", "World!".parse().unwrap());

        Response::ok("returned your headers to you.").map(|res| res.with_headers(headers.into()))
    })?;

    router.on_async("/formdata-name", |mut req, _env, _params| async move {
        let form = req.form_data().await?;

        if !form.has("name") {
            return Response::error("Bad Request", 400);
        }

        Response::ok(format!("key: `name`: {:?}", form.get("name").unwrap()))
    })?;

    router.on("/user/:id/test", |req, _env, params| {
        if !matches!(req.method(), Method::Get) {
            return Response::error("Method Not Allowed", 405);
        }
        if let Some(id) = params.get("id") {
            return Response::ok(format!("TEST user id: {}", id));
        }

        Response::error("Error", 500)
    })?;

    router.on("/user/:id", |_req, _env, params| {
        let id = params.get("id").unwrap_or("not found");
        Response::from_json(&User {
            id: id.into(),
            timestamp: Date::now().as_millis(),
            date_from_int: Date::new(DateInit::Millis(1234567890)).to_string(),
            date_from_str: Date::new(DateInit::String(
                "Wed Jan 14 1980 23:56:07 GMT-0700 (Mountain Standard Time)".into(),
            ))
            .to_string(),
        })
    })?;

    router.post("/account/:id/zones", |_, _, params| {
        Response::ok(format!(
            "Create new zone for Account: {}",
            params.get("id").unwrap_or("not found")
        ))
    })?;

    router.get("/account/:id/zones", |_, _, params| {
        Response::ok(format!(
            "Account id: {}..... You get a zone, you get a zone!",
            params.get("id").unwrap_or("not found")
        ))
    })?;

    router.on_async("/async", |mut req, _env, _params| async move {
        Response::ok(format!("Request body: {}", req.text().await?))
    })?;

    router.on_async("/fetch", |_req, _env, _params| async move {
        let req = Request::new("https://example.com", Method::Post)?;
        let resp = Fetch::Request(&req).send().await?;
        let resp2 = Fetch::Url("https://example.com").send().await?;
        Response::ok(format!(
            "received responses with codes {} and {}",
            resp.status_code(),
            resp2.status_code()
        ))
    })?;

    router.on_async("/fetch_json", |_req, _env, _params| async move {
        let data: ApiData = Fetch::Url("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .await?
            .json()
            .await?;
        Response::ok(format!(
            "API Returned user: {} with title: {} and completed: {}",
            data.user_id, data.title, data.completed
        ))
    })?;

    router.on_async("/proxy_request/*url", |_req, _env, params| {
        // Must copy the parameters into the heap here for lifetime purposes
        let url = params
            .get("url")
            .unwrap()
            .strip_prefix('/')
            .unwrap()
            .to_string();
        async move { Fetch::Url(&url).send().await }
    })?;

    router.on_async("/durable", |_req, env, _params| async move {
        let namespace = env.durable_object("COUNTER")?;
        let stub = namespace.id_from_name("A")?.get_stub()?;
        stub.fetch_with_str("/").await
    })?;

    router.get("/secret", |_req, env, _params| {
        Response::ok(env.secret("SOME_SECRET")?.to_string())
    })?;

    router.get("/var", |_req, env, _params| {
        Response::ok(env.var("SOME_VARIABLE")?.to_string())
    })?;

    router.on_async("/kv", |_req, env, _params| async move {
        let kv = env.kv("SOME_NAMESPACE")?;
        kv.put("another-key", "another-value")?.execute().await?;

        Response::empty()
    })?;

    router.run(req, env).await
}
