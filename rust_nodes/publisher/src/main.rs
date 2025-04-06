use rclrs;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = rclrs::Context::default();
    let node = rclrs::create_node(&context, "rust_publisher")?;
    let publisher = node.create_publisher::<std_msgs::msg::String>("chatter", rclrs::QOS_PROFILE_DEFAULT)?;

    // Get 5 parameters from environment
    let message_prefix = env::var("MESSAGE_PREFIX").unwrap_or("Hello".to_string());
    let message_suffix = env::var("MESSAGE_SUFFIX").unwrap_or("World".to_string());
    let rate = env::var("RATE").unwrap_or("1".to_string()).parse::<u64>().unwrap_or(1);
    let repeat = env::var("REPEAT").unwrap_or("10".to_string()).parse::<u32>().unwrap_or(10);
    let debug = env::var("DEBUG").unwrap_or("false".to_string()) == "true";

    for i in 0..repeat {
        let msg = std_msgs::msg::String { data: format!("{} {} [{}]", message_prefix, message_suffix, i) };
        if debug {
            println!("Publishing: {}", msg.data);
        }
        publisher.publish(msg)?;
        thread::sleep(Duration::from_secs(rate));
    }

    Ok(())
}
