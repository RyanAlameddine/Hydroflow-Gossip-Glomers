mod protocol;

use hydroflow::{tokio, hydroflow_syntax, serde_json::{self, from_str}};

use crate::protocol::Message;



#[tokio::main]
async fn main() {

    //let x : String = Default::default();

    //let y = from_str(x);

    let mut flow = hydroflow_syntax! {
        inbound_chan = source_stdin() -> map(Result::unwrap) -> map(|x| from_str(&x)) -> map(Result::unwrap);

        inbound_chan -> for_each(|x : Message| println!("{:?}", x));   
    };

    flow.run_async().await;
}