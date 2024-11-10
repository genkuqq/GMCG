pub fn handle_chat(variant: u8,data: &[u8]){
    match variant{
        1 => {
            println!("Message.");
        }
        _ => {
            return;
        }
    }
    print!("Chat Packet {} {:?}",variant,data)
}
