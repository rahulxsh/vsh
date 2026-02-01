use std::error::Error;
use vsh::kvm::system::Kvm;

fn main() {
    let kvm = Kvm::open();

    match kvm {
        Ok(d) => {
            println!("FD is : {}", d.fd());
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            println!("Source: {:?}", e.source());
        }
    }
}