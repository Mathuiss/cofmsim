use std::{fs::File, sync::mpsc::Receiver, thread};

use csv::Writer;

use crate::models::BattleResult;

pub fn spawn_writer(channel: Receiver<BattleResult>, mut writer: Writer<File>) {
    thread::spawn(move || {
        let mut handle = channel.iter();

        while let Some(result) = handle.next() {
            _ = writer.serialize(result);
        }
    });
}
