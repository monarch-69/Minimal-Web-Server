pub mod threadpool;
pub mod uri;

use threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };
use std::fs::File;
use std::sync::Arc;
use uri::uri_parser;


fn get_arg() -> String
{
    let flag = std::env::args().nth(1);
    match flag
    {
        Some(flag) => flag.trim().to_string(),
        None => { println!("Please specify something!");
            std::process::exit(1) }
    } 
}

fn help_flag()
{
    println!("test => To check the server after installation");
    println!("start => To start the server");
    println!("stop => To stop the server");
    println!("help => List all the commands with small summary");
    println!("For more info read the guide.txt in the home folder.");
    std::process::exit(1);
}

fn parse_path(buffer: &[u8], path_vector: &Vec<String>) -> (String, String)
{ 
    if buffer.starts_with(b"GET / HTTP/1.1\r\n")
    {
        return ("HTTP/1.1 200 OK\r\n\r\n".to_string(), "static/index.html".to_string());
    }
    else
    {   
        let mut vector = path_vector.iter();
        while let Some(path) = vector.next()
        {
            if buffer.starts_with(format!("GET /{} HTTP/1.1\r\n", path).as_bytes())
            {
                return ("HTTP/1.1 200 OK\r\n\r\n".to_string(), format!("static/{}", path.clone()));
            }
        }
    }
    return ("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(), "error/404.html".to_string());
}

// A function to read from the stream and print out the data.
fn handle_connection(mut stream: TcpStream, path_vector: Arc<Vec<String>>)
{
    // Buffer to store the incoming data
    let mut buffer = [0; 1024];

    // Reading from the stream
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = parse_path(&buffer, &path_vector);

    // Creating a file type variable and a variable to store the read contents of the file.
    let mut file = File::open(filename.as_str()).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    // Writing our response with the contents of the file.
    let response: String = format!("{}{}", status_line.as_str(),file_contents);

    // Sending our response to the browser.
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() 
{
    let _pool = ThreadPool::new(5);
    let mut counter: u8 = 0;
    let path = uri_parser();
    let path_vector = Arc::new(path);

    

    // Now that we have created a listener, now shall we start listening to the binded port for incoming streams.
    let arg = get_arg();
    match arg.as_str()
    {
        "test" =>
        {
            // Very first thing which should be done is
            // a listener must be created to listen the incoming TCP traffic.
            let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
            println!("Congratulations on your successful install!\nPaste this link in your browser : http://127.0.0.1:6969/\nif you don't see the content right away then try reloading the webpage...");
            let mut file_content = String::new();
            File::open("test/hello.html").unwrap().read_to_string(&mut file_content).unwrap();
            for stream in listener.incoming()
            {
                let mut stream = stream.unwrap();
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nCache-Control: no-cache\r\n\r\n{}",file_content).as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        },
        "start" =>
        {
            // Very first thing which should be done is
            // a listener must be created to listen the incoming TCP traffic.
            let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
            for stream in listener.incoming()
            {
                if counter < 5
                {
                    counter += 1;
                    let stream = stream.unwrap();
                    let path = path_vector.clone();
                    std::thread::spawn(move || {
                        handle_connection(stream, path)
                });
                }
                else
                {
                    println!("We are at the maximum capacity!");
                    break;
                }
            }
        },
        "stop" => { println!("Shutting down the server!"); std::process::exit(1); },
        "help" => { help_flag(); },
        _ => { println!("Not yet implemented!"); std::process::exit(1); }
    };
     
}