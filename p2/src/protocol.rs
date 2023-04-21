use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Body {
    init(InitMsg),
    init_ok(InitOkMsg),
    error(ErrorMsg),

    //-- uid -- 
    generate(GenerateMsg),
    generate_ok(GenerateOkMsg),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitMsg {
    pub msg_id: i32,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitOkMsg {
    pub in_reply_to: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMsg {
    pub in_reply_to: i32,
    pub code: i32,
    pub text: String,
}


// -- ECHO -- 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateMsg {
    pub msg_id: Option<i32>,
    pub in_reply_to: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOkMsg {
    pub msg_id: Option<i32>,
    pub in_reply_to: Option<i32>,
    pub id: String//Id
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    pub node_id: String,
    pub msg_id: i32
}