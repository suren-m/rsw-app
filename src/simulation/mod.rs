use std::{collections::HashMap, sync::mpsc, thread, time::Duration};

use chrono::Utc;
use log::info;
use rand::Rng;
use rsw_lib::{
    device::{DeviceKind, Platform},
    events::UserEvent,
    user::{UserFactory, UserId},
};
use thread::JoinHandle;

pub struct Simulation {
    user_factory: UserFactory,
    active_users: HashMap<UserId, DeviceKind<Platform>>,
}

impl Simulation {
    pub fn new(user_factory: UserFactory) -> Self {
        Simulation {
            user_factory,
            active_users: HashMap::new(),
        }
    }

    pub fn run(mut self) -> Vec<JoinHandle<()>> {
        let max_limit = self.user_factory.get_users().len();

        let (tx, rx) = mpsc::channel();
        let tx2 = mpsc::Sender::clone(&tx);

        // logins emitter
        let tx1_handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let rand_user_id = rng.gen_range(1..=max_limit);

                // odd = ios; even = android
                let platform = if rand_user_id % 2 == 0 {
                    Platform::Android
                } else {
                    Platform::IOS
                };

                let device_kind = DeviceKind::SmartPhone(platform);

                let event =
                    UserEvent::Login(device_kind, rand_user_id, Utc::now());

                tx.send(event).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // logouts emitter
        let tx2_handle = thread::spawn(move || loop {
            let mut rng = rand::thread_rng();
            loop {
                let rand_user_id = rng.gen_range(1..=max_limit);

                // odd = ios; even = android
                let platform = if rand_user_id % 2 == 0 {
                    Platform::Android
                } else {
                    Platform::IOS
                };

                let device_kind = DeviceKind::SmartPhone(platform);
                let event =
                    UserEvent::Logout(device_kind, rand_user_id, Utc::now());

                tx2.send(event).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        let rx_handle = thread::spawn(move || {
            for event in rx {
                info!("{:?}", event);
                self.user_factory.update_event(event);
            }
        });

        vec![tx1_handle, tx2_handle, rx_handle]
    }
}

// userdata <Id, User>

// pub trait Emitter {
//     fn emit<T>();
// }

// pub struct LoginEmitter {}

// impl Emitter for LoginEmitter {
//     fn emit<LoginEmitter>() {
//         // spit out some user logins
//     }
// }

// pub struct LogoutEmitter {}

// impl Emitter for LogoutEmitter {
//     fn emit<LogoutEmitter>() {
//         // spit out some user logouts
//     }
// }
