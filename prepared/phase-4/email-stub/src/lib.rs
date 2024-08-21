#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
struct Component;
impl crate::bindings::exports::demo::email_stub::stub_email::Guest for Component {
    type Api = crate::Api;
}
impl crate::bindings::exports::demo::email_stub::stub_email::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_send_email(
        &self,
        list_uri: crate::bindings::demo::email::api::Uri,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:email/api.{send-email}",
                &[WitValue::builder().record().item().string(&list_uri.value).finish()],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:email/api.{send-email}"
                ),
            );
        ()
    }
    fn send_email(&self, list_uri: crate::bindings::demo::email::api::Uri) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:email/api.{send-email}",
                &[WitValue::builder().record().item().string(&list_uri.value).finish()],
            )
            .expect(
                &format!("Failed to invoke remote {}", "demo:email/api.{send-email}"),
            );
        ()
    }
}
bindings::export!(Component with_types_in bindings);
