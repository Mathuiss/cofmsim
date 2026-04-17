use std::{
    fs::File,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use csv::Writer;

use crate::models::BattleResult;

/// Spawns a background thread that writes a record to the output `.csv` file,
/// every time a `BattleResult` is sent through the `mspc` channel.
pub fn spawn_writer(channel: Receiver<BattleResult>, mut writer: Writer<File>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut handle = channel.iter();

        while let Some(result) = handle.next() {
            _ = writer.serialize(result);
        }
    })
}
