use smash::app::{self, BattleObject, BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector4f};
use smash::hash40;
use smash::lua2cpp::L2CFighterCommon;
use crate::modules::*;
use utils_dyn::consts::*;
use utils_dyn::ext::*;
use smash::phx::Vector3f;
use crate::util;

pub unsafe fn update() {
    // skip this frame because the match hasnt started
    if !app::sv_information::is_ready_go() {
        return;
    }

    //println!("doing turbo update!");
    for i in 0..8 {
        if let Some(fighter) = util::get_fighter_common_from_entry_id(i) {
            handle_turbo(fighter);
        }

    }
}

unsafe fn handle_turbo(fighter: &mut L2CFighterCommon) {
    //println!("doing turbo logic");
    if AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT) {
        // enable turbo behavior
        CancelModule::enable_cancel(fighter.boma());
        //println!("enabled cancelling!");

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.sub_wait_ground_check_common(false.into());
        } else {
            fighter.sub_air_check_fall_common();
        }
    }
}