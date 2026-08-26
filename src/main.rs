use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;

//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    //Removed the hard-code port '8000' it's coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}

