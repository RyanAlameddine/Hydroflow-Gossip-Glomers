mod protocol;

use hydroflow::{tokio, hydroflow_syntax, serde_json::{self, from_str}};

use crate::protocol::{Message, Body, InitOkMsg, InitMsg, EchoOkMsg};
//{"src": "c1", "dest": "n3", "body": {"type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"]}}
//./maelstrom/maelstrom test -w echo --bin ./target/debug/hydroflow-template.exe --time-limit 5

#[tokio::main]
async fn main() {

    let mut flow = hydroflow_syntax! {
        inbound_chan = source_stdin() -> map(Result::unwrap) -> map(|x| from_str(&x)) -> map(Result::unwrap);

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
        init[0] -> map(|(dest, body) : (String, InitMsg)| 
            Message {
                src: body.node_id, 
                dest, 
                body: Body::init_ok(InitOkMsg {in_reply_to: body.msg_id})
            }) 
            -> for_each(|x| println!("{}",  serde_json::to_string(&x).unwrap()));
          
        //Save node id for later use
        node_id = init[1] -> map(|(_, body) : (_, InitMsg)| body.node_id);
        
        id_echo = cross_join();
        node_id        -> [0]id_echo;
        channels[echo] -> [1]id_echo;

        id_echo -> enumerate() -> map(|(msg_id, (node_id, (dest, body)))| 
            Message {
                src: node_id, 
                dest, 
                body: Body::echo_ok(EchoOkMsg {
                    msg_id: Some(msg_id),
                    in_reply_to: body.msg_id,
                    echo: body.echo
                })
            }) 
            -> for_each(|x| println!("{}",  serde_json::to_string(&x).unwrap()));

        //out = channels[echo];// -> map(|(x, y)| );   
        
        //out -> for_each(|x| println!("Echo: {:?}", x));

    };

    flow.run_async().await;
}