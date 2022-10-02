use chaos_communicator::communicator::ChaosCommunicator;
use chaos_communicator::message::ChaosMessage;

use crate::world::ChaosWorld;

pub trait ChaosSystem {

    fn init(&mut self, world: &mut ChaosWorld);
    fn subscribe(&self, communicator: &mut ChaosCommunicator);

    fn update(&mut self) -> Vec::<ChaosMessage>;
}

