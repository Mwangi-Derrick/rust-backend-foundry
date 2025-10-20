mod services {
    pub mod upload;
    pub mod payment;
}

fn main() {
    println!("🚀 Microservice starting...");
    services::upload::start_upload();
    services::payment::process_payment();
    println!("✅ All services operational!");
}
