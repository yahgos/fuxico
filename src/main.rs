use std::io::stdin;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: BODY,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde (untagged)]
enum BODY {
    RPC(RPC),
    RESPONSES(RESPONSES),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum RPC {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        echo: String,
        msg_id: usize,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum RESPONSES {
    InitOk {
        in_reply_to: usize,
    },
    Error {
        in_reply_to: usize,
        code: u16,
        text: String,
    },
    EchoOk {
        echo: String,
        msg_id: usize,
        in_reply_to: usize,
    },
}

fn main() {
    let handler = stdin();

    let mut buffer = String::new();
    match handler.read_line(&mut buffer) {
        Ok(_) => {
            // println!("{buffer}");

            let msg: Message = serde_json::from_str(&buffer).unwrap();
            // println!("first test {:?}", msg);
            match msg.body {
                BODY::RPC(rpc) => {
                    let response_message: Message = build_response(rpc, msg.dst, msg.src);
                    send_response(response_message);

                }
                BODY::RESPONSES(response) => {
                    println!("response: {:?}", response)
                }
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn build_response(rpc: RPC, src: String, dst: String) -> Message {
    let response_body: BODY = match rpc {
        RPC::Init {
            msg_id,
            node_id: _,
            node_ids: _,
        } => {
            let response = RESPONSES::InitOk {
                in_reply_to: msg_id,
            };
            BODY::RESPONSES(response)
        }
        RPC::Echo { echo, msg_id } => {
            let response = RESPONSES::EchoOk {
                echo: echo,
                msg_id: msg_id,
                in_reply_to: msg_id,
            };
            BODY::RESPONSES(response)
        }
    };
    Message {
        src: src,
        dst: dst,
        body: response_body,
    }
}

fn send_response(response_message: Message) {
    let response_string = serde_json::to_string(&response_message).unwrap();
    println!("{}", response_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_response() {
        let rpc = RPC::Init {
            msg_id: 123,
            node_id: "123".to_string(),
            node_ids: vec!["123".to_string()],
        };
        let src = "123".to_string();
        let dst = "123".to_string();
        let response = build_response(rpc, src, dst);
        println!("response: {:?}", response);
    }

    #[test]
    fn test_send_response() {
        let rpc = RPC::Init {
            msg_id: 123,
            node_id: "123".to_string(),
            node_ids: vec!["123".to_string()],
        };
        let src = "123".to_string();
        let dst = "123".to_string();
        let response = build_response(rpc, src, dst);
        send_response(response);
    }

    #[test]
    fn test_send_response_echo() {
        let rpc = RPC::Echo {
            echo: "123".to_string(),
            msg_id: 123,
        };
        let src = "123".to_string();
        let dst = "123".to_string();
        let response = build_response(rpc, src, dst);
        send_response(response);
    }

    #[test]
    fn test_serialize() {
        let rpc = RPC::Echo {
            echo: "123".to_string(),
            msg_id: 123,
        };
        let src = "123".to_string();
        let dst = "123".to_string();
        let response = build_response(rpc, src, dst);
        let response_string = serde_json::to_string(&response).unwrap();
        println!("response: {}", response_string);
    }

    #[test]
    fn test_deserialize() {
        let rpc = RPC::Echo {
            echo: "123".to_string(),
            msg_id: 123,        };
        let src = "123".to_string();
        let dst = "123".to_string();
        let response = build_response(rpc, src, dst);
        let response_string = serde_json::to_string(&response).unwrap();
        let response: Message = serde_json::from_str(&response_string).unwrap();
        println!("response: {:?}", response);
}
}
