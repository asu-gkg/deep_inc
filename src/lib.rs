mod server;
mod config;

#[cfg(test)]
mod tests {
    #[test]
    fn test_udp_comm() {
        println!("hello")
    }
}

#[cfg(test)]
mod simulation {
    use crate::config::config::Config;

    #[test]
    fn simulate_it() -> turmoil::Result {
        // build the simulation
        let mut sim = turmoil::Builder::new().build();

        // set up a server
        sim.client("server", async {
            // host software goes here
            println!("server say hi");
            let conf = Config::new(true, 0);


            Ok(())
        });

        // set up the test
        sim.client("test", async {
            // dns lookup for "server"
            let addr = turmoil::lookup("server");
            println!("addr: {}", addr);
            // test code goes here
            Ok(())
        });

        // run the simulation
        sim.run()
    }
}