use std::io::{Stdin, StdoutLock, Write};
use anyhow::{Context};

use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
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
    InitOk,
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                // respone to init message
                // {
                //     "type":        "init_ok",
                //     "in_reply_to": 1
                // }
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body { id: Some(self.id), in_reply_to: input.body.id, payload: Payload::InitOk  }

                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to init")?;
                // de-referencing and create a new mutable ref

                output.write_all(b"\n").context("write trailing newline")?;

                self.id += 1;
            }
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body { id: Some(self.id), in_reply_to: input.body.id, payload: Payload::EchoOk { echo }  }

                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to init")?;
                // de-referencing and create a new mutable ref

                output.write_all(b"\n").context("write trailing newline")?;

                self.id += 1;
            }

            Payload::EchoOk { echo } => {}

            Payload::InitOk { .. } => {}
        };


        Ok(())

    }
}
//
// #[derive(Debug, Clone, Deserialize, Serialize)]
// struct Person {
//     #[serde(flatten)]
//     name: Name,
//     #[serde(rename = "number")]
//     age: usize
// }

// #[derive(Deserialize, Serialize, Clone, Debug)]
// #[serde(tag = "type")]
// #[serde(rename_all = "snake_case")]
// enum Name {
//     MyNhi {name: String},
//     DennisWei { name: String }
// }



fn main() -> anyhow::Result<()> {

    // let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";
    //
    // let stream = Deserializer::from_str(data).into_iter::<Value>();
    //
    // for value in stream {
    //     println!("{}", value.unwrap());
    // }

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>(); // need to specify type of into_inter

    let mut stdout = std::io::stdout().lock();
    // let mut output = serde_json::Serializer::new(stdout);

    let mut state = EchoNode { id: 0 };


    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
        state
            .step(input, &mut stdout)
            .context("Node step function failed")?;
    }

    Ok(())
}
