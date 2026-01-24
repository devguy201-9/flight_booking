use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sea_orm::DatabaseConnection;

use crate::infrastructure::messaging::kafka::message::{Action, KafkaMessage};

#[tracing::instrument(skip_all)]
pub async fn kafka_consumer_task(con: StreamConsumer, _db: DatabaseConnection) {
    con.subscribe(&["channels", "departments"])
        .expect("Failed to subscribe to topics");

    tracing::info!("Starting the consumer loop...");

    loop {
        match con.recv().await {
            Err(e) => tracing::warn!("Kafka error: {}", e),
            Ok(m) => {
                let Some(payload) = m.payload() else {
                    tracing::error!("Could not find a payload :(");
                    continue;
                };

                // here we use `from_slice()` as we initally send it as &[u8]
                let message: KafkaMessage = match serde_json::from_slice(payload) {
                    Ok(res) => res,
                    Err(e) => {
                        // if there is a deserialization error, print an error
                        // and go to the next loop iteration
                        tracing::error!("Deserialization error: {e}");
                        continue;
                    }
                };

                match message.action {
                    Action::CreateChannel => {
                        // TODO: handle
                    }
                    _ => {}
                }
                // Action::UpdateProgramFromRescheduleCommandHandler => {
                //     let data: UpdateProgramCommand =
                //         serde_json::from_value(message.data).unwrap();
                //     let detail_frame_service = DetailFrameService::new(None, None);
                //     let tx = db.begin().await.expect("Failed to begin transaction");
                //     match detail_frame_service
                //         .update_detail_frame_from_reschedule_command_handler(
                //             &tx, message.id, &data,
                //         )
                //         .await
                //     {
                //         Ok(_) => {
                //             tx.commit().await.expect("Failed to commit transaction");
                //             tracing::info!(
                //                 "Successfully updated traffic from reschedule command handler"
                //             );
                //         },
                //         Err(e) => {
                //             tx.rollback().await.expect("Failed to rollback transaction");
                //             tracing::error!("Failed to update traffic: {}", e);
                //         },
                //     }
                // },

                // print out our payload

                let _ = con
                    .store_offset_from_message(&m)
                    .inspect_err(|e| tracing::warn!("Error while storing offset: {}", e));
            }
        };
    }
}
