use crate::messages;
use std::time::Duration;

pub async fn stream_direction() {
    use messages::direction_messages::*;

    let mut current_number: i32 = 1;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        PlayerDirection { current_number }.send_signal_to_dart();
        current_number = ({ current_number + 1 } % 4);
    }
}

pub async fn get_direction() {
    
}