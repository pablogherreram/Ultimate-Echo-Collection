#![feature(proc_macro_hygiene)]

use std::collections::HashMap;

const TARGET_UI_CHARA: &str = "ui_chara_pickel";
const CUSTOM_CHARACALL: &str = "vc_narration_characall_pablo3xdd";

// Hash40 keys for characall_label_c00 through characall_label_c07.
const CALL_C00: u64 = 0x1337FC912E;
const CALL_C01: u64 = 0x1340FBA1B8;
const CALL_C02: u64 = 0x13D9F2F002;
const CALL_C03: u64 = 0x13AEF5C094;
const CALL_C04: u64 = 0x1330915537;
const CALL_C05: u64 = 0x13479665A1;
const CALL_C06: u64 = 0x13DE9F341B;
const CALL_C07: u64 = 0x13A998048D;

fn check_dependencies() -> bool {
    let dependencies = [
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
    ];

    let mut passed = true;
    for dependency in dependencies {
        if !std::path::Path::new(dependency).is_file() {
            skyline::println!(
                "[Pablo3xDD C04] Missing dependency: {}",
                dependency
            );
            passed = false;
        }
    }
    passed
}

fn call(label: &str) -> the_csk_collection_api::Hash40Type {
    the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40(label)))
}

extern "C" fn mods_mounted(_event: arcropolis_api::Event) {
    skyline::println!("[Pablo3xDD C04] Registering individual announcer call");

    // CSK Collection resolves this label to:
    // vc_narration/vc_narration_characall_pablo3xdd.idsp
    the_csk_collection_api::add_narration_characall_entry(CUSTOM_CHARACALL);

    // Preserve every original Steve/Alex/Zombie/Enderman call explicitly,
    // changing only Steve's C04 label.
    let labels = HashMap::from([
        (CALL_C00, call("vc_narration_characall_pickel")),
        (CALL_C01, call("vc_narration_characall_pickel_c01")),
        (CALL_C02, call("vc_narration_characall_pickel")),
        (CALL_C03, call("vc_narration_characall_pickel_c01")),
        (CALL_C04, call(CUSTOM_CHARACALL)),
        (CALL_C05, call("vc_narration_characall_pickel_c01")),
        (CALL_C06, call("vc_narration_characall_pickel_c06")),
        (CALL_C07, call("vc_narration_characall_pickel_c07")),
    ]);

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: smash::hash40(TARGET_UI_CHARA),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(labels),
            ..Default::default()
        }
    );

    skyline::println!(
        "[Pablo3xDD C04] Applied {} to ui_chara_pickel C04",
        CUSTOM_CHARACALL
    );
}

#[skyline::main(name = "pablo3xdd_announcer_c04")]
pub fn main() {
    if !check_dependencies() {
        skyline::println!("[Pablo3xDD C04] Plugin disabled because dependencies are missing");
        return;
    }

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                event: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }

        arcrop_register_event_callback(
            arcropolis_api::Event::ModFilesystemMounted,
            mods_mounted,
        );
    }
}
