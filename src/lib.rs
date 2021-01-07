use std::{cell::RefCell, collections::HashMap, rc::Rc, thread};

use log::{debug, info, warn};
use rsw_lib::{device::{DeviceKind, Platform}, events::UserEvent, user::{UserFactory, UserId}};
use simulation::{ Simulation};

mod simulation;
pub type ActiveUsers = Rc<RefCell<HashMap<UserId, DeviceKind<Platform>>>>;

pub fn begin_simulation() {
    let mut user_factory = UserFactory::new(10).create_users(7).unwrap();
    let max_users = user_factory.get_users().len();

    let active_users: ActiveUsers = Rc::new(RefCell::new(HashMap::new()));
     
    let sim = Simulation::new();
    for event in sim.run(max_users) {
        debug!("{:?}", event);
        match event {
            UserEvent::Login(device_kind, user_id, _) => {
                active_users.borrow_mut().insert(user_id, device_kind);
            },
            UserEvent::Logout(_, user_id, _) => {
                if active_users.borrow_mut().contains_key(&user_id) {
                    active_users.borrow_mut().remove(&user_id);
                    warn!("user {} logged out", user_id);
                }      
            },
            _ => {}
        }
        user_factory.update_event(event.clone());
        let total_active = active_users.borrow().len(); 
        info!("total active:{}", total_active);
    }

    info!("..done..");
}
