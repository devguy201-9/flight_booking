use std::sync::Arc;

use rdkafka::producer::FutureProducer;

use crate::infrastructure::messaging::kafka::event::{
    kafka_address_event_publisher::KafkaAddressEventPublisher,
    kafka_airport_event_publisher::KafkaAirportEventPublisher,
    kafka_boarding_pass_event_publisher::KafkaBoardingPassEventPublisher,
    kafka_booking_event_publisher::KafkaBookingEventPublisher,
    kafka_checkin_event_publisher::KafkaCheckinEventPublisher,
    kafka_flight_event_publisher::KafkaFlightEventPublisher,
    kafka_passenger_event_publisher::KafkaPassengerEventPublisher,
    kafka_user_event_publisher::KafkaUserEventPublisher,
};

pub struct EventPublishers {
    pub user: Arc<KafkaUserEventPublisher>,
    pub address: Arc<KafkaAddressEventPublisher>,
    pub airport: Arc<KafkaAirportEventPublisher>,
    pub flight: Arc<KafkaFlightEventPublisher>,
    pub booking: Arc<KafkaBookingEventPublisher>,
    pub passenger: Arc<KafkaPassengerEventPublisher>,
    pub checkin: Arc<KafkaCheckinEventPublisher>,
    pub boarding_pass: Arc<KafkaBoardingPassEventPublisher>,
}

pub fn build_event_publishers(producer: Arc<FutureProducer>) -> EventPublishers {
    EventPublishers {
        user: Arc::new(KafkaUserEventPublisher::new(producer.clone())),
        address: Arc::new(KafkaAddressEventPublisher::new(producer.clone())),
        airport: Arc::new(KafkaAirportEventPublisher::new(producer.clone())),
        flight: Arc::new(KafkaFlightEventPublisher::new(producer.clone())),
        booking: Arc::new(KafkaBookingEventPublisher::new(producer.clone())),
        passenger: Arc::new(KafkaPassengerEventPublisher::new(producer.clone())),
        checkin: Arc::new(KafkaCheckinEventPublisher::new(producer.clone())),
        boarding_pass: Arc::new(KafkaBoardingPassEventPublisher::new(producer)),
    }
}
