use flarecord::models::command::SubcommandType;
use flarecord::prelude::*;
use flarecord::command;




#[command]
impl Command for Ext {
    fn name(&self) -> String {
        "ext".into()
    }

    fn description(&self) -> String {
        "Set of commands to manage extensions".into()
    }

    fn subcommands(&self) -> Vec<SubcommandType> {
        vec![
            
        ]
    }
}