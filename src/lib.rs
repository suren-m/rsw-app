use log::info;
use rsw_lib::user::UserFactory;
use simulation::Simulation;

mod simulation;

pub fn begin_simulation() {
    let mut user_factory = UserFactory::new(1000).create_users(750).unwrap();
    let max_users = user_factory.get_users().len();

    let sim = Simulation::new();

    for event in sim.run(max_users) {
        info!("{:?}", event);
        user_factory.update_event(event);
    }

    info!("..done..");
}
