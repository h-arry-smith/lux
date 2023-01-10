use lumen::output::NetworkState;
use std::{
    net::ToSocketAddrs,
    sync::Mutex,
    time::{Duration, Instant},
};
use tauri::Manager;

use lumen::{
    output::{sacn::ACN_SDT_MULTICAST_PORT, NetworkOutput},
    universe::Multiverse,
};
use tauri::{plugin::Builder, plugin::TauriPlugin, Runtime};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("network")
        .setup(|app_handle| {
            let mut network = Network::new();
            network.bind("127.0.0.1:12345");
            network.connect(format!("127.0.0.1:{}", ACN_SDT_MULTICAST_PORT));
            app_handle.manage(Mutex::new(network));
            Ok(())
        })
        .build()
}

pub struct Network {
    output: NetworkOutput,
    last_connection_attempt: Instant,
}

impl Network {
    fn new() -> Self {
        Self {
            output: NetworkOutput::new(),
            last_connection_attempt: Instant::now(),
        }
    }

    fn bind(&mut self, addr: impl ToSocketAddrs) {
        match self.output.bind(addr) {
            Ok(()) => println!("bound to address"),
            Err(e) => eprintln!("could not bind: {e}"),
        }
    }

    pub fn try_connect(&mut self, addr: impl ToSocketAddrs) {
        if self.last_connection_attempt.elapsed() > Duration::new(0, 1_000_000_000 / 2) {
            self.connect(addr);
        }
    }

    fn connect(&mut self, addr: impl ToSocketAddrs) {
        self.last_connection_attempt = Instant::now();
        match self.output.connect(addr) {
            Ok(()) => println!("connected to sacn..."),
            Err(e) => println!("failed to connect to sacn: {e}"),
        }
    }

    pub fn output_multiverse(&mut self, multiverse: &Multiverse) {
        for universe in multiverse.universes() {
            self.output.send_data(universe);
        }
    }

    pub fn state(&self) -> NetworkState {
        self.output.state
    }
}
