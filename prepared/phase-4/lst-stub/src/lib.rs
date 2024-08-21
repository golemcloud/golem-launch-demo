#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
pub struct FutureGetResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FuturePollResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureConnectResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureConnectedEditorsResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureIsArchivedResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct EmailQuery {
    rpc: WasmRpc,
}
impl EmailQuery {}
pub struct FutureDeadlineResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureRecipientsResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::demo::lst_stub::stub_lst::Guest for Component {
    type Api = crate::Api;
    type FutureGetResult = crate::FutureGetResult;
    type FuturePollResult = crate::FuturePollResult;
    type FutureConnectResult = crate::FutureConnectResult;
    type FutureConnectedEditorsResult = crate::FutureConnectedEditorsResult;
    type FutureIsArchivedResult = crate::FutureIsArchivedResult;
    type EmailQuery = crate::EmailQuery;
    type FutureDeadlineResult = crate::FutureDeadlineResult;
    type FutureRecipientsResult = crate::FutureRecipientsResult;
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureGetResult
for FutureGetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!("Failed to invoke remote {}", "demo:lst/api.{get}"),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| {
                        item.string().expect("string not found").to_string()
                    })
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFuturePollResult
for FuturePollResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<crate::bindings::demo::lst::api::Change>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!("Failed to invoke remote {}", "demo:lst/api.{poll}"),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| {
                        let (case_idx, inner) = item
                            .variant()
                            .expect("variant not found");
                        match case_idx {
                            0u32 => {
                                crate::bindings::demo::lst::api::Change::Added(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            1u32 => {
                                crate::bindings::demo::lst::api::Change::Deleted(
                                    inner
                                        .expect("variant case not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                )
                            }
                            2u32 => {
                                crate::bindings::demo::lst::api::Change::Inserted({
                                    let record = inner.expect("variant case not found");
                                    crate::bindings::demo::lst::api::InsertParams {
                                        after: record
                                            .field(0usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                        value: record
                                            .field(1usize)
                                            .expect("record field not found")
                                            .string()
                                            .expect("string not found")
                                            .to_string(),
                                    }
                                })
                            }
                            _ => unreachable!("invalid variant case index"),
                        }
                    })
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureConnectResult
for FutureConnectResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<crate::bindings::demo::lst::api::Connection> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!("Failed to invoke remote {}", "demo:lst/api.{connect}"),
                    );
                ({
                    let record = result.tuple_element(0).expect("tuple not found");
                    crate::bindings::demo::lst::api::Connection {
                        id: record
                            .field(0usize)
                            .expect("record field not found")
                            .u64()
                            .expect("u64 not found"),
                    }
                })
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureConnectedEditorsResult
for FutureConnectedEditorsResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "demo:lst/api.{connected-editors}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| {
                        item.string().expect("string not found").to_string()
                    })
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureIsArchivedResult
for FutureIsArchivedResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<bool> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}", "demo:lst/api.{is-archived}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .bool()
                    .expect("bool not found"))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_add(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
        value: String,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{add}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(
                &format!("Failed to invoke-and-await remote {}", "demo:lst/api.{add}"),
            );
        ()
    }
    fn add(&self, c: crate::bindings::demo::lst::api::Connection, value: String) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:lst/api.{add}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(&format!("Failed to invoke remote {}", "demo:lst/api.{add}"));
        ()
    }
    fn blocking_delete(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
        value: String,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{delete}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(
                &format!("Failed to invoke-and-await remote {}", "demo:lst/api.{delete}"),
            );
        ()
    }
    fn delete(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
        value: String,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:lst/api.{delete}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(&format!("Failed to invoke remote {}", "demo:lst/api.{delete}"));
        ()
    }
    fn blocking_insert(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
        after: String,
        value: String,
    ) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{insert}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&after),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(
                &format!("Failed to invoke-and-await remote {}", "demo:lst/api.{insert}"),
            );
        ()
    }
    fn insert(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
        after: String,
        value: String,
    ) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:lst/api.{insert}",
                &[
                    WitValue::builder().record().item().u64(c.id).finish(),
                    WitValue::builder().string(&after),
                    WitValue::builder().string(&value),
                ],
            )
            .expect(&format!("Failed to invoke remote {}", "demo:lst/api.{insert}"));
        ()
    }
    fn blocking_get(&self) -> Vec<String> {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/api.{get}", &[])
            .expect(
                &format!("Failed to invoke-and-await remote {}", "demo:lst/api.{get}"),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.string().expect("string not found").to_string())
            .expect("list not found"))
    }
    fn get(
        &self,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureGetResult {
        let result = self.rpc.async_invoke_and_await("demo:lst/api.{get}", &[]);
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureGetResult::new(FutureGetResult {
            future_invoke_result: result,
        })
    }
    fn blocking_poll(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
    ) -> Vec<crate::bindings::demo::lst::api::Change> {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{poll}",
                &[WitValue::builder().record().item().u64(c.id).finish()],
            )
            .expect(
                &format!("Failed to invoke-and-await remote {}", "demo:lst/api.{poll}"),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| {
                let (case_idx, inner) = item.variant().expect("variant not found");
                match case_idx {
                    0u32 => {
                        crate::bindings::demo::lst::api::Change::Added(
                            inner
                                .expect("variant case not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                        )
                    }
                    1u32 => {
                        crate::bindings::demo::lst::api::Change::Deleted(
                            inner
                                .expect("variant case not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                        )
                    }
                    2u32 => {
                        crate::bindings::demo::lst::api::Change::Inserted({
                            let record = inner.expect("variant case not found");
                            crate::bindings::demo::lst::api::InsertParams {
                                after: record
                                    .field(0usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                                value: record
                                    .field(1usize)
                                    .expect("record field not found")
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            }
                        })
                    }
                    _ => unreachable!("invalid variant case index"),
                }
            })
            .expect("list not found"))
    }
    fn poll(
        &self,
        c: crate::bindings::demo::lst::api::Connection,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FuturePollResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "demo:lst/api.{poll}",
                &[WitValue::builder().record().item().u64(c.id).finish()],
            );
        crate::bindings::exports::demo::lst_stub::stub_lst::FuturePollResult::new(FuturePollResult {
            future_invoke_result: result,
        })
    }
    fn blocking_connect(
        &self,
        email: String,
    ) -> crate::bindings::demo::lst::api::Connection {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{connect}",
                &[WitValue::builder().string(&email)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:lst/api.{connect}"
                ),
            );
        ({
            let record = result.tuple_element(0).expect("tuple not found");
            crate::bindings::demo::lst::api::Connection {
                id: record
                    .field(0usize)
                    .expect("record field not found")
                    .u64()
                    .expect("u64 not found"),
            }
        })
    }
    fn connect(
        &self,
        email: String,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureConnectResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "demo:lst/api.{connect}",
                &[WitValue::builder().string(&email)],
            );
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureConnectResult::new(FutureConnectResult {
            future_invoke_result: result,
        })
    }
    fn blocking_disconnect(&self, c: crate::bindings::demo::lst::api::Connection) -> () {
        let result = self
            .rpc
            .invoke_and_await(
                "demo:lst/api.{disconnect}",
                &[WitValue::builder().record().item().u64(c.id).finish()],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:lst/api.{disconnect}"
                ),
            );
        ()
    }
    fn disconnect(&self, c: crate::bindings::demo::lst::api::Connection) -> () {
        let result = self
            .rpc
            .invoke(
                "demo:lst/api.{disconnect}",
                &[WitValue::builder().record().item().u64(c.id).finish()],
            )
            .expect(&format!("Failed to invoke remote {}", "demo:lst/api.{disconnect}"));
        ()
    }
    fn blocking_connected_editors(&self) -> Vec<String> {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/api.{connected-editors}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "demo:lst/api.{connected-editors}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.string().expect("string not found").to_string())
            .expect("list not found"))
    }
    fn connected_editors(
        &self,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureConnectedEditorsResult {
        let result = self
            .rpc
            .async_invoke_and_await("demo:lst/api.{connected-editors}", &[]);
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureConnectedEditorsResult::new(FutureConnectedEditorsResult {
            future_invoke_result: result,
        })
    }
    fn blocking_archive(&self) -> () {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/api.{archive}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:lst/api.{archive}"
                ),
            );
        ()
    }
    fn archive(&self) -> () {
        let result = self
            .rpc
            .invoke("demo:lst/api.{archive}", &[])
            .expect(&format!("Failed to invoke remote {}", "demo:lst/api.{archive}"));
        ()
    }
    fn blocking_is_archived(&self) -> bool {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/api.{is-archived}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "demo:lst/api.{is-archived}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .bool()
            .expect("bool not found"))
    }
    fn is_archived(
        &self,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureIsArchivedResult {
        let result = self.rpc.async_invoke_and_await("demo:lst/api.{is-archived}", &[]);
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureIsArchivedResult::new(FutureIsArchivedResult {
            future_invoke_result: result,
        })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureDeadlineResult
for FutureDeadlineResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Option<u64>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "demo:lst/email-query.{deadline}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .option()
                    .expect("option not found")
                    .map(|inner| inner.u64().expect("u64 not found")))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestFutureRecipientsResult
for FutureRecipientsResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Vec<String>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "demo:lst/email-query.{recipients}"
                        ),
                    );
                (result
                    .tuple_element(0)
                    .expect("tuple not found")
                    .list_elements(|item| {
                        item.string().expect("string not found").to_string()
                    })
                    .expect("list not found"))
            })
    }
}
impl crate::bindings::exports::demo::lst_stub::stub_lst::GuestEmailQuery for EmailQuery {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_deadline(&self) -> Option<u64> {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/email-query.{deadline}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "demo:lst/email-query.{deadline}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .option()
            .expect("option not found")
            .map(|inner| inner.u64().expect("u64 not found")))
    }
    fn deadline(
        &self,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureDeadlineResult {
        let result = self
            .rpc
            .async_invoke_and_await("demo:lst/email-query.{deadline}", &[]);
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureDeadlineResult::new(FutureDeadlineResult {
            future_invoke_result: result,
        })
    }
    fn blocking_recipients(&self) -> Vec<String> {
        let result = self
            .rpc
            .invoke_and_await("demo:lst/email-query.{recipients}", &[])
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "demo:lst/email-query.{recipients}"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .list_elements(|item| item.string().expect("string not found").to_string())
            .expect("list not found"))
    }
    fn recipients(
        &self,
    ) -> crate::bindings::exports::demo::lst_stub::stub_lst::FutureRecipientsResult {
        let result = self
            .rpc
            .async_invoke_and_await("demo:lst/email-query.{recipients}", &[]);
        crate::bindings::exports::demo::lst_stub::stub_lst::FutureRecipientsResult::new(FutureRecipientsResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
