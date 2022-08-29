use midir::MidiInput;
use std::env::args;

fn main() {
    let a: Vec<String> = args().collect();
    let input = MidiInput::new("midicheap").expect("failed to create midi input!");
    let ports = input.ports();

    if a.len() == 1 {
        for (idx, p) in ports.iter().enumerate() {
            println!("{} {}", idx, input.port_name(&p).expect("failed to retrieve the name of a port!"))
        }
    }
    else if a.len() == 2 {
        let idx = a[1].parse::<usize>().expect("failed to parse index!");
        let port = &ports[idx];
        println!("{}", input.port_name(port).expect("failed to retrieve the name of a port!"));
        let _connection = input.connect(port, "midicheap input", |ts, msg, _| print_msg(ts, msg), ());
        loop {}
    }
}

fn print_msg(ts: u64, msg: &[u8]) {
    if msg.len() == 3 {
        println!("{:016x}  {:02x}  {:02}  {:02x}  {:02x}", ts, msg[0] & 0xf0, (msg[0] & 0x0f) + 1, msg[1], msg[2])
    }
    else {
        //println!("{:016x}  {:?}", ts, msg)
    }
}
