use std::ops::Add;
use std::time::{Duration, SystemTime};
use crate::bindings::demo::lst_stub::stub_lst::EmailQuery;
use crate::bindings::exports::demo::email::api::{Guest, Uri};

mod bindings;


struct Component;

impl Guest for Component {
    fn send_email(list_uri: bindings::golem::rpc::types::Uri) {
        let list_uri = list_uri.into();
        loop {
            if let Some(deadline) = get_deadline(&list_uri) {
                let now = SystemTime::now();
                if deadline < now {
                    send_emails(&list_uri)
                } else {
                    let to_sleep = deadline.duration_since(now).expect("deadline is in the past");
                    std::thread::sleep(to_sleep);
                }
            } else {
                println!("No deadline for {list_uri:?}, cancelling");
                break;
            }
        }
    }
}

fn send_emails(list_uri: &Uri) {
    println!("Getting email addresses from {list_uri:?}");
    let query = EmailQuery::new(list_uri);
    let emails = query.blocking_recipients();
    for email in emails {
        println!("Sending email to {email}");
    }
}

fn get_deadline(list_uri: &Uri) -> Option<SystemTime> {
    println!("Getting deadline from {list_uri:?}");

    let query = EmailQuery::new(list_uri);
    let epoch_ms = query.blocking_deadline();

    println!("Got deadline from list: {epoch_ms:?}");

    epoch_ms.map(|ms| SystemTime::UNIX_EPOCH.add(Duration::from_millis(ms)))
}
bindings::export!(Component with_types_in bindings);
