use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Body {
    init(InitMsg),
    init_ok(InitOkMsg),
    error(ErrorMsg),

    //-- ECHO -- 
    echo(EchoMsg),
    echo_ok(EchoOkMsg),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitMsg {
    msg_id: i32,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitOkMsg {
    in_reply_to: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorMsg {
    in_reply_to: i32,
    code: i32,
    text: String,
}


// -- ECHO -- 
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoMsg {
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    echo: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoOkMsg {
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    echo: String
}