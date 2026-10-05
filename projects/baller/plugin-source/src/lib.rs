#![feature(proc_macro_hygiene)]
#![allow(
    unused_macros
)]

use std::collections::HashMap;



//=========================================================================================================================

// Your ui_chara ID here :
pub const YOUR_CHARA_ID : &str = "ui_chara_baller";

// The name ID you want :
pub const YOUR_NAME_ID : &str = "baller";

// Your chara_layout ID here :
pub const YOUR_CHARA_LAYOUT : &str = "ui_chara_baller_00";

// The ui_chara ID it is based on :
pub const BASE_CHARA_ID : &str = "ui_chara_tantan";

// The fighter_kind it is based on :
pub const BASE_FIGHTER_KIND : &str = "fighter_kind_tantan";

// The Series your character come from
pub const CHARA_SERIES : &str = "ui_series_arms"; // If you're not sure, just put "mario"

// The characall of the base character
pub const BASE_CHARACALL : &str = "vc_narration_characall_tantan";

// If you have a personnalized vc_narration_characall type true, else, let it to false
pub const IS_CHARA_CALL: bool = false;

// Your narration_characall ID here if you have one :
pub const YOUR_CHARACALL : &str = "vc_narration_characall_baller";

//=========================================================================================================================



pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

pub fn check_deps() -> bool {
    let mut passed = true;

    for dep in [
        "rom:/skyline/plugins/libparam_config.nro",
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    ] {
        if !std::path::Path::new(dep).is_file() {
            println!("{} not found! This installation is incomplete. Please download all dependencies listed in the README file.", dep);
            passed = false;
        }
    }

    passed
}


extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    // EVERYTHING HERE IS UNIQUE THAT'S MEANT TO BE MODIFIED FOR YOUR SPECIFIC PLUGIN.

    const FIGHTER_NAME: &str = "tantan"; // The BASE Character
    const MARKER_FILE: &str = "baller.marker"; // YOUR Character marker
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }



    let _color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    // Preform csk collection calls here (allow online, narration characall entry, add chara db, add layout db, w/e)

    the_csk_collection_api::allow_ui_chara_hash_online(smash::hash40(YOUR_CHARA_ID));

    let actual_characall: &'static str = if IS_CHARA_CALL {
        the_csk_collection_api::add_narration_characall_entry(YOUR_CHARACALL);
        YOUR_CHARACALL
    } else {
        BASE_CHARACALL
    };

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry { 
            ui_chara_id: smash::hash40(YOUR_CHARA_ID) /* Hash40 of the new ui_chara ID */, 
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new(YOUR_NAME_ID)), /* Your new name ID */
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40(BASE_FIGHTER_KIND) /* Hash40 of the base fighter_kind */), 
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40(BASE_FIGHTER_KIND) /* Hash40 of the base fighter_kind */)), 
            ui_series_id: the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40(CHARA_SERIES) /* Hash40 of the ui_series */)), 
            fighter_type: the_csk_collection_api::Hash40Type::Optional(Some(0x1353795179 /* Hash40 of fighter_type_normal */)), 
            alt_chara_id: the_csk_collection_api::Hash40Type::Optional(Some(0x0)), 
            exhibit_year: the_csk_collection_api::ShortType::Optional(Some(2006)), 
            exhibit_day_order: the_csk_collection_api::IntType::Optional(Some(91303)), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Optional(Some(0)), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_img_ext_skill_page1: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_img_ext_skill_page2: the_csk_collection_api::BoolType::Optional(Some(false)), 
            skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(20)), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(20)), 
            save_no: the_csk_collection_api::SignedByteType::Optional(Some(20)), 
            chara_count: the_csk_collection_api::SignedByteType::Optional(Some(1)), 
            can_select: the_csk_collection_api::BoolType::Optional(Some(true)), 
            is_usable_soundtest: the_csk_collection_api::BoolType::Optional(Some(true)), 
            is_called_pokemon: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_mii: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_hidden_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_dlc: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_patch: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_plural_message: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_plural_narration: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_article: the_csk_collection_api::BoolType::Optional(Some(false)), 
            extra_flags: the_csk_collection_api::IntType::Optional(Some(0)), 
            result_pf0: the_csk_collection_api::BoolType::Optional(Some(true)), 
            result_pf1: the_csk_collection_api::BoolType::Optional(Some(true)), 
            result_pf2: the_csk_collection_api::BoolType::Optional(Some(true)), 
            color_num: the_csk_collection_api::UnsignedByteType::Optional(Some(_color_num as u8)), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Optional(Some(0x0)), 
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                    (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40(actual_characall)))),
                    (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
                    (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))),
                    (0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40(BASE_CHARA_ID))),
            ])), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                    (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9F873561A /* Hash40 of c00_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x95E045DAE /* Hash40 of c01_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x96FEC4733 /* Hash40 of c02_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9C99B4C87 /* Hash40 of c03_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x90C3C7209 /* Hash40 of c04_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x9AA4B79BD /* Hash40 of c05_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x99BA36320 /* Hash40 of c06_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                    (0x93DD46894 /* Hash40 of c07_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))),
                    (0x11895f00fc, the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
            ])), 
            ..Default::default()
        }
    );

/* REMOVE THIS LINE HERE !!!!!!!!!!

    the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
        // Paste your code here
    });

REMOVE THIS LINE HERE !!!!!!!!!!*/

}


#[skyline::main(name = "plugin")]
pub fn main() {

    if !check_deps() {
        return; // don't do anything else since they don't have all dependencies
    }

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}
