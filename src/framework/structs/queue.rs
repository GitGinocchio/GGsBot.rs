use worker::*;

use crate::QUEUES;

pub struct QueueProcessor {
    env: Env,
    ctx: Context,
}

impl QueueProcessor {
    pub fn new(env: Env, ctx: Context) -> Self {
        Self { env, ctx }
    }

    pub async fn process(&self, batch: MessageBatch<serde_json::Value>) -> Result<()> {
        let queue_name = batch.queue();
        worker::console_log!("[QueueJob]: Batch started for queue '{}'", queue_name);

        if let Some(handler) = QUEUES.get(&queue_name) {
            worker::console_log!(
                "[QueueJob]: Executing handler for queue '{}'...",
                queue_name
            );

            if let Err(e) = handler.handle(batch, &self.env, &self.ctx).await {
                worker::console_error!(
                    "[QueueJob]: Handler for queue '{}' FAILED with error: {:?}",
                    queue_name,
                    e
                );
                return Err(e.into());
            }

            worker::console_log!(
                "[QueueJob]: Handler for queue '{}' finished successfully.",
                queue_name
            );
        } else {
            worker::console_warn!(
                "[QueueJob]: No handler registered for queue: '{}'",
                queue_name
            );
        }

        worker::console_log!(
            "[QueueJob]: Batch completed successfully for queue '{}'",
            queue_name
        );
        Ok(())
    }
}
