use async_trait::async_trait;

#[async_trait(?Send)]
pub trait CommandEvents {
    /// method called when this command is set up on a discord server
    async fn before_setup(&self) {
        // come parametri qui dovrebbero essere passati i parametri passati come inizializzazione
        // del comando

        // prima di questo bisogna creare come in GGsBot le pagine diname per il setup
    }
    async fn after_setup(&self) {
    }

    /// method called when a command is removed from a discord server (act like a clean-up)
    async fn before_teardown(&self) {}
    async fn after_teardown(&self) {}

    /// method called when a command is enabled from a discord server
    async fn on_enabled(&self) {}

    /// method called when a command is disabled from a discord server
    async fn on_disabled(&self) {}
}