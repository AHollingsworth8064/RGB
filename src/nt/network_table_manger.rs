use std::{
    borrow::Cow, collections::{HashMap, HashSet, VecDeque}, hash::Hash, net::{Ipv4Addr, SocketAddr, IpAddr}, ops::Div, sync::{Arc, Weak}, time::{Duration, Instant}, vec,
};

use tokio::{
    net::TcpStream,
    select,
    sync::{mpsc, oneshot, Mutex},
    task::yield_now,
};


struct NetworkTableClient{
    decode_buf : Vec<u8>,
    connection : TcpStream,
    clock_offset : u32,
    clock_synced : bool,
    next_pub_uid : u32,
    next_sub_uid : u32,
}

impl NetworkTableClient{
    pub(in crate::nt) async fn new(
        ip_addr : Ipv4Addr,
        port : u16,
    ) -> NetworkTableClient{
    
    let socket_addr = SocketAddr::new(IpAddr::V4((ip_addr)), port);
    
    //done for the day //  Need to set up the connection bs 

    return NetworkTableClient{
        decode_buf :vec![],
        connection : TcpStream::connect(SocketAddr::new(IpAddr::V4((Ipv4Addr::new(10,0,0,10))),1001)).await.unwrap(),
        clock_offset : 0,
        clock_synced : false,
        next_pub_uid : 0,
        next_sub_uid : 0,
    }
    }
}


pub struct NetworkTable<'a >{
    client : NetworkTableClient<'a>,
    pub subscribers : Vec<String>,
    pub publishers : Vec<String>,
    pub topics : HashMap<String, String>
}