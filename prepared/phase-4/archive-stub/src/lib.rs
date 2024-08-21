#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureGetAllResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::demo::archive_stub::stub_archive::Guest for Component {
    type Api = crate::Api;
    type FutureGetAllResult = crate::FutureGetAllResult;
}
impl crate::bindings::exports::demo::archive_stub::stub_archive::GuestFutureGetAllResult
for FutureGetAllResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<crate::bindings::demo::archive::api::ArchivedList>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "demo:archive/api.{get-all}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| {
                        let record = item;
                        crate::bindings::demo::archive::api::ArchivedList {
                            name: record
                                .field(0usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            items: record
                                .field(1usize)
                                .expect("record field not found")
                                .list_elements(|item| {
                                    item.string().expect("string not found").to_string()
                                })
                                .expect("list not found"),
                        }
                    })
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::demo::archive_stub::stub_archive::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_store(&self, name: String, items: Vec<String>) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:archive/api.{store}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder()
                        .list_fn(
                            &items,
                            |item, item_builder| { item_builder.string(item) },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:archive/api.{store}"
                ),
            );
        ()
    }
    fn store(&self, name: String, items: Vec<String>) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:archive/api.{store}",
                &[
                    WitValue::builder().string(&name),
                    WitValue::builder()
                        .list_fn(
                            &items,
                            |item, item_builder| { item_builder.string(item) },
                        ),
                ],
            )
            .expect(&format!("Failed to invoke remote {}", "demo:archive/api.{store}"));
        ()
    }
    fn blocking_get_all(
        &self,
    ) -> Vec<crate::bindings::demo::archive::api::ArchivedList> {
        let result = self
            .rpc
            .invoke_and_await("demo:archive/api.{get-all}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:archive/api.{get-all}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let record = item;
                crate::bindings::demo::archive::api::ArchivedList {
                    name: record
                        .field(0usize)
                        .expect("record field not found")
                        .string()
                        .expect("string not found")
                        .to_string(),
                    items: record
                        .field(1usize)
                        .expect("record field not found")
                        .list_elements(|item| {
                            item.string().expect("string not found").to_string()
                        })
                        .expect("list not found"),
                }
            })
            .expect("list not found"))
    }
    fn get_all(
        &self,
    ) -> crate::bindings::exports::demo::archive_stub::stub_archive::FutureGetAllResult {
        let result = self.rpc.async_invoke_and_await("demo:archive/api.{get-all}", &[]);
        crate::bindings::exports::demo::archive_stub::stub_archive::FutureGetAllResult::new(FutureGetAllResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
