use dotenv::dotenv;
use hyper::body::Buf;
use hype::{header, body, client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

//structs to work with API response 
#[derive(Deserialize, Debug)]
struct OAIResponse{
    id: Option<String>,
    object:Option<String>,
    created:Option<String>,
    model:Option<String>,
    choices: Vec<OAIChoices>,

}

//struct for the  choices
#[derive(Deserialize, Debug)]
 struct OAIChoices{
    text:String,
    index:u8,
    logprobs: Option<u8>,
    finish_reason:String,
 }
//structs for the request you will make to the api

#[derive(Deserialize, Debug)]
struct OAIResquest{
    prompt: String,
    max_tokens: u16,
}

//tokio async main fn
asyn fn main()-> Result<(), Box<dyn std::error::error+send+sync>>{
    //load my env variable
    dotenv().ok();
    //create a Httpsconnector, hyper
    let https = HttpsConnector::new();
    //create a client
    let client =Client::builder().build(https);
    let uri ="https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble ="generate an sql command for a given command";
    let oai_token :String=env::var("OIA_TOKEN").unwrap();
    let auth_header_val = fomart!("Bearer{}", oia_token);
    println!("{esc}c", esc=)
}



//URL to which we will make the request
//preamblee, prompt to chatGPT
//token, in the header
//loop, inside the loop away to read user input
//spinner, wait for the response
//request to chatGpt  for every single user input, loop
//response and we print the response