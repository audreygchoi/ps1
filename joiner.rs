use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>, <inputfile>", args[0]); 
    } else {
        let share1 = args[1].clone();
	let share2 = args[2].clone();
        let path1 = Path::new(share1.clone());
	let path2 = Path::new(share2.clone());
        let msg_file1 = File::open(&path1);
	let msg_file2 = File::open(&path2);

        match (msg_file1, msg_file2) {
            (Some(mut msg1),Some( mut msg2)) => {
		let msg1_bytes : ~[u8] = msg1.read_to_end();
		let msg2_bytes : ~[u8] = msg2.read_to_end();
                let textfile
		       = File::create(&Path::new((share1.slice(0,(share1.len()-7)) + ".txt"))); 
                match (textfile) {
                    Some(mut text) => { 
                        joiner(msg1_bytes, msg2_bytes, text); 
                        } ,
                    None => fail!("Error opening output files!"),
                }
            } , 
            (_, _) => { fail!("Error opening message file");}
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn joiner(msg1_bytes: &[u8], msg2_bytes: &[u8], mut text: File) {
    let unencrypted_bytes = xor(msg1_bytes, msg2_bytes);
    text.write(unencrypted_bytes);
}
