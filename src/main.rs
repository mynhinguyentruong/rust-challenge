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
    in_reply_to: Option<RequestId>,
    #[serde(flatten)]
    payload: Payload
}

type RequestId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // turn all Echo to snake_case
enum Payload {
    Echo { echo: String, },
    EchoOk { echo: String }
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    fn step(&mut self, input: Message, output: &mut serde_json::Serializer<StdoutLock>) -> anyhow::Result<()> {
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
        };

        
    Ok(())

    }
}



fn main() -> anyhow::Result<()> {
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
