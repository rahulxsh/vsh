pub mod errors;
pub mod kvm;

use std::error::Error;
use kvm::system::Kvm;

fn open_kvm_file(){
    let kvm = Kvm::open();
    
    match kvm { 
        Ok(d) =>{
            println!("FD is : {}",d.get_kvm_fd());
        },
        Err(e) =>{
            println!("Error: {}",e.to_string());
            println!("Source: {:?}",e.source());
        }
    }
}