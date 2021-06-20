use std::{collections::HashMap, time::Duration};

use mini_redis::{Command, Connection, Frame, Result, client::{self, connect}};
use tokio::{net::{TcpListener, TcpStream}, task, time::sleep};

#[tokio::main]
pub async fn main()-> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:6379")
    .await
    .unwrap();

    loop{
        let (socket,_) = listener
        .accept()
        .await
        .unwrap();
        process(socket)
        .await;
        
    }

}

async fn process(socket: TcpStream){

    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);


    while let Some(frame) = connection
    .read_frame()
    .await
    .unwrap(){
        
        let response = match Command::from_frame(frame).unwrap(){
            Command::Set(cmd) =>{
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }   
            Command::Get(cmd)=>{
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }else{
                    Frame::Null
                }
            }  
            cmd => panic!("unimplemented {:?}",cmd)
        };
        
        connection.write_frame(&response)
        .await
        .unwrap();
    }
}


// not use Tokio
// use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread::sleep, time::{self, Duration, SystemTime}};

// fn main(){
//     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
//     loop{
//         let (socket,_) = listener.accept().unwrap();
//         process(socket);

//     }

// }

// pub fn process(mut socket:TcpStream){
//     // println!("{:?}",SystemTime::now());
//     sleep(Duration::new(10, 0));
// }