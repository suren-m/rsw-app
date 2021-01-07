use log::info;
use rsw_lib::user::UserFactory;
use simulation::Simulation;

mod simulation;

pub fn begin_simulation() {
    let user_factory = UserFactory::new(1000).create_users(750).unwrap();

    let sim = Simulation::new(user_factory);
    let handles = sim.run();

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    info!("..done..");
}
