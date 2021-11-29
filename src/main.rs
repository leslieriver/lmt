use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

mod models;

mod format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage; cargo run <server> <community> <limit>");
        std::process::exit(1);
    }

    let server = &args[1];
    let community = &args[2];
    let limit = args[3]
        .parse::<u32>()
        .expect("parsing failed for 3rd argument");

    println!("server {}", server);
    println!("community {}", community);

    let client = reqwest::Client::builder().build()?;

    let mut counter = 1;

    // Delete html file if it doesn't exist;
    let dir_name = format!("./{}", community);
    let index_file = format!("./{}/index.html", community);
    let _ = fs::create_dir_all(&dir_name);
    let _ = fs::remove_file(&index_file);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&index_file)
        .unwrap();

    let page = format!(
        r#"<!DOCTYPE html>
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

    "#,
        community
    );

    let _ = write!(file, "{}", &page);

    //let mut retry_count = 0;
    let query = format!(
        "{}/api/v3/post/list?community_name={}&limit={}",
        server, community, limit
    );

    let res = client.get(query).send().await?;

    if res.status() == 200 {
        // retry_count = 0;

        let posts = res.json::<models::Root>().await?;

        if posts.posts.len() < 1 {
            println!("No posts found");
            std::process::exit(1);
        } else {
            for p in posts.posts.iter() {
                println!("Backing up post {}", counter);
                counter = counter + 1;
                if let Err(e) = writeln!(file, "<h1>{}</h1>", &p.post.name) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                match &p.post.body {
                    Some(body) => {
                        let _ = writeln!(file, "<p>{}</p>", &body);
                    }
                    _ => {}
                }
                match &p.post.url {
                    Some(url) => {
                        let _ = writeln!(file, "<a href={}>{}</a>", &url, &url);
                        let _ = writeln!(file, "<br><br>");
                        let response = reqwest::get(url).await?;
                        let ending = format::return_ending(url.to_string());
                        if ending != "".to_string() {
                            if ending == "mp4".to_string() {
                                let video_tag = format!(
                                    r#" <video width="320" height="240" controls>
                                        <source src="./{}.mp4" type="video/mp4">
                                        Your browser does not support the video tag.
                                    </video> "#,
                                    counter
                                );

                                let _ = writeln!(file, "{}", video_tag);
                            } else {
                                let _ = writeln!(
                                    file,
                                    "<img src=./{}.{} alt={}>",
                                    counter, ending, counter
                                );
                            }
                            let filename = format!("./{}/{}.{}", community, counter, ending);
                            let path = Path::new(&filename);
                            let mut file = match File::create(&path) {
                                Err(why) => panic!("couldn't create {}", why),
                                Ok(file) => file,
                            };
                            let content = response.bytes().await?;
                            file.write_all(&content)?;
                        }
                    }
                    _ => {}
                }
            }
        }
    } else {
        println!("request failed. Are your server and community details correct?");
        std::process::exit(1);
    }

    let footer = "</body>
    </html>";
    let _ = write!(file, "{}", &footer);

    Ok(())
}
