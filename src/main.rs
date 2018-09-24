extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map = HashMap::new();


    match args.len()
    {
        3 => // Enough args: continue
        {
            map.insert("USERNAME", &args[0]);
            map.insert("PASSWORD", &args[1]);

            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();

            loop
            {
                let res = client.post("https://smoothwall.chhs.org.uk:442/clogin")
                    .json(&map)
                    .send();

                match res
                {
                    Ok(_) =>
                    {
                        let resp = res.unwrap();

                        match resp.status()
                        {
                            reqwest::StatusCode::OK => println!("All ok"),

                            s => println!("Issue: {:?}", s)
                        };

                    },

                    Err(a) =>
                    {
                        println!("An error occured: {:?}", a)
                    },
                };

                thread::sleep(time::Duration::from_secs(120));
            }
        },

        _ => { // Any other number of args: cancel
            println!("Please provide your username and password as an argument.");
            return;
        }
    };
}