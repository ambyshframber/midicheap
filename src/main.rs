use midir::MidiInput;
use std::env::args;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e)
    }
}

fn run() -> Result<(), &'static str> {
    let a: Vec<String> = args().collect();
    let input = MidiInput::new("midicheap").map_err(|_| "failed to create midi input!")?;
    let ports = input.ports();

    if a.len() == 1 {
        for (idx, p) in ports.iter().enumerate() {
            println!("{} {}", idx, input.port_name(&p).unwrap_or("???".into()))
        }
    }
    else if a.len() == 2 {
        let idx = a[1].parse::<usize>().map_err(|_| "failed to parse index!")?;
        let port = &ports[idx];
        println!("{}", input.port_name(port).unwrap_or("???".into()));
        let _connection = input.connect(port, "midicheap input", print_msg, ());
        loop {}
    }

    Ok(())
}

fn print_msg(ts: u64, msg: &[u8], _: &mut ()) {
    //println!("{:016x}  {:02x}  {:02}  {:02x}  {:02x}", ts, msg[0] & 0xf0, (msg[0] & 0x0f) + 1, msg[1], msg[2])

    let msg_type = (msg[0] >> 4) & 0b111;
    let msg_type_display = msg[0] & 0xf0;
    let channel = (msg[0] & 0x0f) + 1;
    match msg_type {
        0b000..=0b011 => { // note on/off, poly aftertouch, cc
                   //   ts     msg type  ch     d1      d2   name
            println!("{:016x}  {:02x}  {:02}  {:02x}  {:02x}  {}", ts, msg_type_display, channel, msg[1], msg[2], CHANNEL_MSG_NAMES[msg_type as usize])

            // TODO: channel mode
        }
        0b100 => { // program change

        }
        0b101 => { // channel aftertouch
            
        }
        0b110 => { // pitchbend
            let bend_amt = (msg[1] as u16) | ((msg[2] as u16) << 7);
            println!("{:016x}  {:02x}  {:02}  {:04x}  pitchbend", ts, msg_type_display, channel, bend_amt)
        }
        _ => {

        }
    }
}

const CHANNEL_MSG_NAMES: &[&str] = &["note off", "note on", "polyphonic pressure", "control change"];
