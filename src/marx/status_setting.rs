use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::hooks::InlineCtx;

use crate::common::*;
use crate::common::modules::*;
use crate::marx::*;

pub static mut MARX_4_CUTTER_SETTING : usize = 0x4f9b20;

#[skyline::hook(replace = MARX_4_CUTTER_SETTING)]
pub unsafe fn marx_4_cutter_setting(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) {
        let owner = BossModule::get_owner(module_accessor);
        if WorkModule::is_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES) == false {
            BossModule::install_moves(item,BossKind::MARX);
            WorkModule::on_flag(owner,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        }
    }
    init_status_data(lua_state,ItemKineticType(*ITEM_KINETIC_TYPE_MOTION_LINKED),SituationKind(*SITUATION_KIND_AIR),GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),true);
    return L2CValue::I32(0)
}

pub unsafe fn marx_install_moves(item: &mut L2CAgentBase) {
    install_entry_dead_wait(item);
    install_normals(item);
    install_teleport(item);
    install_movement(item);
}