mod protocol;

use hydroflow::{tokio, hydroflow_syntax, serde_json::{self, from_str}};
use protocol::EchoMsg;

use crate::protocol::{Message, Body, InitOkMsg, InitMsg, EchoOkMsg};
//{"src": "c1", "dest": "n3", "body": {"type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"]}}
//./maelstrom/maelstrom test -w echo --bin ./target/debug/hydroflow-template.exe --time-limit 5

fn create_init_ok((dest, body) : (String, InitMsg)) -> String {
    let message = Message {
        src: body.node_id, 
        dest, 
        body: Body::init_ok(InitOkMsg {in_reply_to: body.msg_id})
    };
    serde_json::to_string(&message).unwrap()
}

fn create_echo_ok((msg_id, (node_id, (dest, body))) : (i32, (String, (String, EchoMsg)))) -> String {
    let message = Message {
        src: node_id, 
        dest, 
        body: Body::echo_ok(EchoOkMsg {
            msg_id: Some(msg_id),
            in_reply_to: body.msg_id,
            echo: body.echo
        })
    };
    serde_json::to_string(&message).unwrap()
}

#[tokio::main]
async fn main() {

    let mut flow = hydroflow_syntax! {
        inbound_chan = source_stdin() -> map(Result::unwrap) -> map(|x| from_str(&x).unwrap());

        channels = inbound_chan -> demux(|o : Message, var_args!(init, error, echo)|
            match o.body {
                Body::init (body) => init .give((o.src, body)),
                Body::echo (body) => echo .give((o.src, body)),
                Body::error(body) => error.give((o.src, body)),
                _ => println!("Found invalid message body: {:?}", o)
            });

        channels[error] -> for_each(|x| println!("Error: {:?}", x));   

        init = channels[init] -> tee();
        //Respond with InitOk
        init[0] -> map(create_init_ok) -> [0]out;
          
        //Save node id for later use
        node_id = init[1] -> map(|(_, body) : (_, InitMsg)| body.node_id);
        
        //cross join node_id with echo channel
        id_echo = cross_join::<'static, 'tick>();
        node_id        -> [0]id_echo;
        channels[echo] -> [1]id_echo;

        //for each echo, generate a response
        id_echo -> enumerate() -> map(|x| create_echo_ok(x)) -> [1]out;


        out = merge() -> for_each(|x| println!("{}",  x));
    };

    flow.run_async().await;
}