use std::io::{Stdin, StdoutLock};
use anyhow::{Context};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")] // type Echo will return echo: echo, type EchoOK will return echo: echo_ok
#[serde(rename_all = "snake_case")] // turn all enum to snake_case 
enum Payload {
    Echo { echo: String, }, // will tag "type": "echo"
    EchoOk { echo: String }, // will tag "type": "echo_ok"
    Init { node_id: String, },
    InitOk { node_id: String },
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut serde_json::Serializer<StdoutLock>) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body { _type: input.body._type, id: Some(self.id), in_reply_to: input.body.id, payload: Payload::EchoOk { echo }  }
        
                };
            reply
                .serialize(output)
                .context("serialize repsonse to echo")?;

            self.id += 1;    
            }

            Payload::EchoOk { echo } => {}

            Payload::Init { node_id } => {}

            Payload::InitOk { node_id } => {}
        };

        
    Ok(())

    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Person {
    #[serde(flatten)]
    name: Name,
    #[serde(rename = "number")]
    age: usize
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Name {
    MyNhi {name: String},
    DennisWei { name: String }
}



fn main() -> anyhow::Result<()> {

    let nhi = Person { age: 1, name: Name::DennisWei { name: "asdasd".to_string() }};

    let nhi_output = serde_json::to_string(&nhi);

    println!("{:?}", nhi_output);

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let stdout = std::io::stdout().lock();
    let mut output = serde_json::Serializer::new(stdout);

    let mut state = EchoNode { id: 0 };


    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
        state
            .step(input, &mut output)
            .context("Node step function failed")?;
    }

    Ok(())
}
