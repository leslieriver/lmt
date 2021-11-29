
use std::{thread, time::Duration};
use std::fs::OpenOptions;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

mod models;

mod format;

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage; cargo run <server> <community>");
        std::process::exit(1);
    }

    let server = &args[1];
    let community = &args[2];

    println!("server {}", server);
    println!("community {}", community);

    let client = reqwest::Client::builder()
        .build()?;

    let mut counter = 1;

    // Delete html file if it doesn't exist;
    let _ = fs::remove_file("index.html");
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("index.html")
    .unwrap();

    let page = format!(r#"<!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8"/>
    <title>{}</title>
    <style>
        img {{
            max-width: 650px;
        }}
    </style>
    </head>
    <body>

    "#,community);

   let _ = write!(file, "{}",&page);

    let mut retry_count = 0;
    loop {
        
        let query = format!("{}/api/v3/post/list?community_name={}&limit=1&page={}",server,community,counter);
        
        let res = client
        .get(query)
        .send()
        .await?;
        
        if  res.status() == 200 {
            retry_count = 0;
            println!("Backing up post {}",counter);
            counter = counter + 1;
            let posts = res
            .json::<models::Root>()
            .await?;

           
            if posts.posts.len() < 1 {
                break;
            }else {
                for p in posts.posts.iter(){

                    if let Err(e) = writeln!(file, "<h1>{}</h1>",&p.post.name) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    match &p.post.body {
                        Some(body) => {
                           let _ = writeln!(file, "<p>{}</p>",&body);
                        },
                        _ =>{},
                    }
                    match &p.post.url {
                        Some(url) => {
                            let _ = writeln!(file, "<a href={}>{}</a>",&url,&url);
                            let _ = writeln!(file, "<br><br>");
                            if url.contains("pictrs/image") {
                                let response = reqwest::get(url).await?;
                                let ending = format::return_ending(url.to_string());
                                if ending == "mp4".to_string() {
                                    let video_tag = format!(r#" <video width="320" height="240" controls>
                                        <source src="{}.mp4" type="video/mp4">
                                        Your browser does not support the video tag.
                                    </video> "#,counter);
                                   
                                   let _ = writeln!(file,"{}",video_tag);
                                }else{
                                    let _ = writeln!(file, "<img src=./{}.{} alt={}>",counter,ending,counter);
                                }
                                let filename = format!("./{}.{}",counter,ending);
                                let path = Path::new(&filename);
                                let mut file = match File::create(&path) {
                                    Err(why) => panic!("couldn't create {}", why),
                                    Ok(file) => file,
                                };
                                let content =  response.bytes().await?;
                                file.write_all(&content)?;
                            }
                           
                           
                        },
                        _ =>{},
                    }
                   

                };
            }
        } else{
            if retry_count < 3{
                retry_count = retry_count + 1;
                println!("Rate limit hit sleeping ten seconds");
                thread::sleep(Duration::from_millis(10000));
            }else {
                println!("retry_count exceded. Are your server and community details correct?");
                break;
            }
        }
    }

    let footer = "</body>
    </html>";
    let _ = write!(file, "{}",&footer);

    Ok(())  
}
