use std::time::Duration;

use worker::*;

use super::SomeSharedData;

#[durable_object]
pub struct AlarmObject {
    state: State,
}

#[durable_object]
impl DurableObject for AlarmObject {
    fn new(state: State, _: Env) -> Self {
        Self { state }
    }

    async fn fetch(&mut self, _: Request) -> Result<Response> {
        let alarmed: bool = match self.state.storage().get("alarmed").await {
            Ok(alarmed) => alarmed,
            Err(e) if e.to_string() == "No such value in storage." => {
                // Trigger our alarm method in 100ms.
                self.state
                    .storage()
                    .set_alarm(Duration::from_millis(100))
                    .await?;

                false
            }
            Err(e) => return Err(e),
        };

        Response::ok(alarmed.to_string())
    }

    async fn alarm(&mut self) -> Result<Response> {
        self.state.storage().put("alarmed", true).await?;

        console_log!("Alarm has been triggered!");

        Response::ok("ALARMED")
    }
}

pub async fn handle_alarm(_req: Request, ctx: RouteContext<SomeSharedData>) -> Result<Response> {
    let namespace = ctx.durable_object("ALARM")?;
    let stub = namespace.id_from_name("alarm")?.get_stub()?;
    // when calling fetch to a Durable Object, a full URL must be used. Alternatively, a
    // compatibility flag can be provided in wrangler.toml to opt-in to older behavior:
    // https://developers.cloudflare.com/workers/platform/compatibility-dates#durable-object-stubfetch-requires-a-full-url
    stub.fetch_with_str("https://fake-host/alarm").await
}

pub async fn handle_id(_req: Request, ctx: RouteContext<SomeSharedData>) -> Result<Response> {
    let namespace = ctx.durable_object("COUNTER").expect("DAWJKHDAD");
    let stub = namespace.id_from_name("A")?.get_stub()?;
    // when calling fetch to a Durable Object, a full URL must be used. Alternatively, a
    // compatibility flag can be provided in wrangler.toml to opt-in to older behavior:
    // https://developers.cloudflare.com/workers/platform/compatibility-dates#durable-object-stubfetch-requires-a-full-url
    stub.fetch_with_str("https://fake-host/").await
}

pub async fn handle_put_raw(req: Request, ctx: RouteContext<SomeSharedData>) -> Result<Response> {
    let namespace = ctx.durable_object("PUT_RAW_TEST_OBJECT")?;
    let id = namespace.unique_id()?;
    let stub = id.get_stub()?;
    stub.fetch_with_request(req).await
}
