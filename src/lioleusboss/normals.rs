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

pub unsafe fn lioleusboss_howling_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("howling"),0.0,2.0,false,0.0,false,false);
    boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_LIOLEUSBOSS),Hash40::new("energy_param_attack_howling"),0.0);
    return L2CValue::I32(0)
}

pub unsafe fn lioleusboss_howling_status(item: &mut L2CAgentBase) -> L2CValue {
    let lua_state = item.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,10.0,false);
        AttackModule::set_power(module_accessor,1,10.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        let owner = BossModule::get_owner(module_accessor);
        WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
        StatusModule::change_status_request(module_accessor,*ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT,false);
    }
    return L2CValue::I32(0)
}

pub static mut LIOLEUSBOSS_BACK_JUMP_ENDING : usize = 0x4ae754;

#[skyline::hook(replace = LIOLEUSBOSS_BACK_JUMP_ENDING, inline)]
pub unsafe fn lioleusboss_back_jump_ending(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *agent_base.module_accessor);
    WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT as u32;
}

pub static mut LIOLEUSBOSS_BACK_JUMP2_ENDING : usize = 0x4af0c4;

#[skyline::hook(replace = LIOLEUSBOSS_BACK_JUMP2_ENDING, inline)]
pub unsafe fn lioleusboss_back_jump2_ending(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *agent_base.module_accessor);
    WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT as u32;
}

pub static mut LIOLEUSBOSS_FIREBALL_ENDING : usize = 0x4ad814;

#[skyline::hook(replace = LIOLEUSBOSS_FIREBALL_ENDING, inline)]
pub unsafe fn lioleusboss_fireball_ending(ctx: &mut InlineCtx) {
    let agent_base: &mut L2CAgentBase = std::mem::transmute(*ctx.registers[20].x.as_ref());
    if WorkModule::is_flag(agent_base.module_accessor,ITEM_INSTANCE_WORK_FLAG_PLAYER) == false {
        return;
    }
    let owner = BossModule::get_owner(&mut *agent_base.module_accessor);
    WorkModule::set_int(owner,*SITUATION_KIND_GROUND,FIGHTER_MARIO_INSTANCE_WORK_ID_INT_BOSS_SITUATION);
    *ctx.registers[0].w.as_mut() = *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT as u32;
}

pub unsafe fn install_normals(item: &mut L2CAgentBase) {
    let lioleusboss_howling_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_howling_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lioleusboss_howling_coroutine_func);
    let lioleusboss_howling_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lioleusboss_howling_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lioleusboss_howling_status_func);
}