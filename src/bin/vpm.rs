use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

const HTTP_BODY: &str = "
    
";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Ok((buff, buf_size)) = read_buff(&args[1]) {
        write_buff(buff, buf_size, &args[1]);
    } else {
        println!("Please enter a valid url");
    }
}

fn read_buff(addr: &str) -> Result<([u8; 1024], u16), std::io::Error> {
    let mut conn = TcpStream::connect(addr).unwrap();
    let mut buff: [u8; 1024] = [0; 1024];
    let buff_size = conn.read(&mut buff)?;
    return Ok((buff, buff_size as u16));
}

fn write_buff(buff: [u8; 1024], buf_length: u16, uri: &str) {
    let buff_adj = &buff[0..buf_length as usize];
    let curr_dir = std::env::current_exe().unwrap();
    let curr_path_split = curr_dir.parent().unwrap().to_str().unwrap();
    let filename = uri.split("/").last().unwrap();

    let mut ptr = File::create(curr_path_split.to_string() + "/vmod_lib/" + filename).unwrap();
    ptr.write(buff_adj).unwrap();
}
