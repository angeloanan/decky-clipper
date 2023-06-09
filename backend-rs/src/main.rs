mod api;

use simplelog::{WriteLogger, LevelFilter};

use usdpl_back::Instance;

const PORT: u16 = 44444;

fn main() -> Result<(), ()> {
    WriteLogger::init(
        #[cfg(debug_assertions)]{LevelFilter::Debug},
        #[cfg(not(debug_assertions))]{LevelFilter::Info},
        Default::default(),
        std::fs::File::create("/tmp/decky-clipper.log").unwrap()
    ).unwrap();

    log::info!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    // let runtime = control::ControlRuntime::new();
    // runtime.run();
    Instance::new(PORT)
        .register("ping", api::ping)
        .register("hello", api::hello)
        .register("version", api::version)
        .register("name", api::name)
        .run_blocking()
}
