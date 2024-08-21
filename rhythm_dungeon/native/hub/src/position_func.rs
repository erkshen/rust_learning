use crate::{direction_func, messages};
use std::time::Duration;

pub async fn stream_position() {
    use messages::position_messages::*;

    // stream the position to dart
    let mut coordinates_x: i32 = 0;
    let mut coordinates_y: i32 = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(10)).await;
        PlayerPosition {coordinates_x, coordinates_y}.send_signal_to_dart();
    }

}

pub async fn move_position() {
    use messages::position_messages::*;
    use messages::direction_messages::*;
    
    
}