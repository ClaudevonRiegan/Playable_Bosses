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
use crate::common::{modules::*,params::*};
use crate::darz::*;

pub unsafe fn darz_move_inputs(fighter_boma: *mut BattleObjectModuleAccessor, input_1: i32, input_2: i32) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let darz_boma = sv_battle_object::module_accessor(boss_id);
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_CROSS_BOMB,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_SPACE_RUSH_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_DARK_PILLAR_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_CENTIPEDE_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_SUMMON_FIGHTER,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_PIERCE_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_TEAR_UP_ANGER,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_DESYNC_POS);
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_GATLING_START,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
    if (input_2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 {
        WorkModule::set_int(darz_boma,*ITEM_DARZ_STATUS_KIND_TELEPORT,ITEM_INSTANCE_WORK_INT_ATTACK_TYPE);
        WorkModule::on_flag(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CHOSEN);
    }
}

pub unsafe fn darz_movement(fighter_boma: *mut BattleObjectModuleAccessor) {
    let boss_id = WorkModule::get_int64(fighter_boma,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_ID) as u32;
    let darz_boma = sv_battle_object::module_accessor(boss_id);
    let move_mul = Darz::move_mul;
    let pos = Vector2f{x: ControlModule::get_stick_x(fighter_boma)*move_mul, y: ControlModule::get_stick_y(fighter_boma)*move_mul};
    PostureModule::add_pos_2d(darz_boma,&pos);
}