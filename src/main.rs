use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

struct Body {
    #[serde(rename == "type")]
    _type: String,
    #[serde(rename == "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<RequestId>,

    rest: HashMap<String, serde_json::Value>
}

type RequestId = usize


fn main() {

}