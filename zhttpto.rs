//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
#[feature(managed_boxes)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::os::*;
use std::io::buffered::*;
static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count: uint = 0;


fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    

    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            unsafe {visitor_count += 1;}
            
	    let mut lines: ~[&str] = request_str.split_str(" ").collect();  
	    let path = lines.remove(1).clone();

	    if path == "/"{
		let response: ~str  =
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body{ background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 <h2>Visitor Count:" + unsafe{ visitor_count.to_str()} + " </h2>
                 </body></html>\r\n";
		 stream.write(response.as_bytes());
		}
	    else {
		 let thepath = path.clone();
	         let mut path_split: ~[&str] = thepath.split_str(".").collect();
		 let file_type = path_split.pop();
		 if file_type == "html"{
			 let file_path = Path::new(path.clone().slice_from(1));
			 match(File::open(&file_path)){
				Some(mut file) => {
					let file_data: ~[u8] = file.read_to_end();
					stream.write(file_data);
				}
				None => {
					fail!("Error opening file!")
				}
			 }
	        }
		else {
		let response: ~str  =
               			 ~"HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                		 <doctype !html><html><head><title>Hello, Rust!</title>
                	 	<style>body{ background-color: #111; color: #FFEEAA }
                        		h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        		h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                		 </style></head>
                	 	<body>
                 		<h1>Uh Oh, Krusty!</h1>
                 		<h2>There was an Error loading your page </h2>
                 		</body></html>\r\n";
                 	stream.write(response.as_bytes());
		}	
	    }
	}
		 	
	   
            println!("Connection terminates.");
        }
    }

