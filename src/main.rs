extern crate serial;

use std::time::Duration;
use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    let mut port = serial::open("/dev/ttyACM0").unwrap();

    //reconfigure port setting
    match port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }){
        Ok(_) => println!("configure ok"),
        Err(e) => return println!("configure error {:?}", e),
    };
	
    match port.set_timeout(Duration::from_millis(2000)){
        Ok(_) => println!("set_timeout ok"),
        Err(e) => return println!("set_timeout error {:?}", e),
    };

    match port.write(b"ATI\r") {
        Ok(_) => println!("Write ok"),
        Err(e) => return println!("Write error {:?}", e),
    };

    let mut retvec: Vec<u8> = Vec::with_capacity(255);

    match port.read(&mut retvec) {
        Ok(_) => println!("Read ok size={:?}, data=({:?})", retvec.len(),  retvec),
        Err(e) => println!("Read error {:?}", e),
    }
}
