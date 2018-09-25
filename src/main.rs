extern crate reqwest;

use std::env;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() == 3
    {
        let handle = thread::spawn(move ||
        {
            let params: [[&str; 2]; 2] = [["USERNAME", &args[1]], ["PASSWORD", &args[2]]];

            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();

            loop
            {
                println!("Logging in now...");
                login(&params, &client);

                thread::sleep(time::Duration::from_secs(120));
            }
        });

        handle.join().unwrap();
    }

    else { // Any other number of args: cancel
        println!("Please provide your username and password as an argument.");
    }
}

fn login(params: &[[&str; 2]; 2], client: &reqwest::Client)
{
    let res = client.post("https://smoothwall.chhs.org.uk:442/clogin")
        .form(&params)
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
}
