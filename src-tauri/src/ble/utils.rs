use btleplug::api::Manager as _;
use btleplug::platform::{Adapter, Manager};
use log::{error, warn};
use uuid::Uuid;

use super::bluetooth::DeviceType;
use super::constants::{
    CYCLING_POWER_MEASUREMENT_UUID, FITNESS_MACHINE_SERVICE_UUID, HEART_RATE_MEASUREMENT_UUID,
    HEART_RATE_SERVICE_UUID, INDOOR_BIKE_DATA_UUID,
};
use super::event_handlers::Characteristic;

pub async fn get_central(manager: &Option<Manager>) -> Option<Adapter> {
    let Some(manager) = manager.as_ref() else {
        warn!("No manager found.");
        return None;
    };

    let Ok(adapters) = manager.adapters().await else {
        warn!("No bluetooth adapters found");
        return None;
    };

    adapters.into_iter().next()
}

pub async fn get_manager() -> Option<Manager> {
    match Manager::new().await {
        Ok(manager) => Some(manager),
        Err(e) => {
            error!("Could not initialize bluetooth manager: {}", e);
            None
        }
    }
}

pub fn get_device_type(services: Vec<Uuid>) -> DeviceType {
    let is_heart_rate = services.contains(&HEART_RATE_SERVICE_UUID);
    let is_smart_trainer = services.contains(&FITNESS_MACHINE_SERVICE_UUID);

    match (is_heart_rate, is_smart_trainer) {
        (true, false) => DeviceType::HeartRate,
        (false, true) => DeviceType::SmartTrainer,
        _ => DeviceType::Generic,
    }
}

pub fn get_uuid_characteristic(uuid: Uuid) -> Characteristic {
    match uuid {
        CYCLING_POWER_MEASUREMENT_UUID => Characteristic::CyclingPowerMeasurement,
        HEART_RATE_MEASUREMENT_UUID => Characteristic::HeartRateMeasurement,
        INDOOR_BIKE_DATA_UUID => Characteristic::IndoorBikeData,
        _ => Characteristic::Unknown,
    }
}
