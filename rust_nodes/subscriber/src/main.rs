use rclrs;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = rclrs::Context::default();
    let node = rclrs::create_node(&context, "rust_subscriber")?;

    let filename = env::var("OUTPUT_FILE").unwrap_or("output.txt".to_string());
    let debug = env::var("DEBUG").unwrap_or("false".to_string()) == "true";

    let _subscription = node.create_subscription::<std_msgs::msg::String, _>(
        "chatter",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg| {
            if debug {
                println!("Received: {}", msg.data);
            }

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filename)
                .expect("Cannot open file");

            writeln!(file, "{}", msg.data).expect("Cannot write to file");
        },
    )?;

    rclrs::spin(&node)?;
    Ok(())
}
