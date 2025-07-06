use std::net::UdpSocket;

pub fn get() -> String {
    // Create a UDP socket and connect to a public DNS server
    // This doesn't actually send any data, but allows us to get the local IP
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            // Connect to Google's public DNS (8.8.8.8:80)
            // This doesn't send data, just determines the route
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => addr.ip().to_string(),
                        Err(_) => "Unknown".to_string(),
                    }
                }
                Err(_) => "Unknown".to_string(),
            }
        }
        Err(_) => "Unknown".to_string(),
    }
}