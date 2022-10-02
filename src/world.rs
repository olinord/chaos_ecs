use std::sync::Arc;
use std::thread;

use chaos_communicator::communicator::ChaosCommunicator;
use chaos_communicator::message::ChaosMessage;

use crate::system::ChaosSystem;

pub struct ChaosWorld{
    systems: Vec<Box<dyn ChaosSystem + Send + Sync>>,
    communicator: Arc<ChaosCommunicator>
}

impl ChaosWorld {
    pub fn new() -> Self {
        ChaosWorld {
            systems: Vec::new(),
            communicator: Arc::new(  ChaosCommunicator::new())
        }
    }

    pub fn with<T: 'static + ChaosSystem + Send + Sync>(&mut self, system: T) -> &mut Self {
         self.systems.push(Box::new(system));
         return self;
    }

    // change the dt to an update context
    pub fn update(&'static mut self, _dt: f32) {
        // Gather up all the components needed for  the system update


        let mut handles = vec![];
        for system in self.systems.iter_mut() {
            handles.push(thread::spawn(move || -> Vec::<ChaosMessage> {
                system.update()
            }));
        }

        // todo: optimize?
        for handle in handles {
            let messages = handle.join().unwrap();
            for message in messages{
                if let Err(_error_message) = self.communicator.send_message(message){
                    println!( "Could not send message" );
                }
            }
        }
    }

}
