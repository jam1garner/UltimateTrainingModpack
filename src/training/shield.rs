use crate::common::consts::*;
use crate::common::*;
use smash::app;
use smash::hash40;
use smash::lib::lua_const::*;

pub unsafe fn get_param_float(
    _module_accessor: &mut app::BattleObjectModuleAccessor,
    param_type: u64,
    param_hash: u64,
) -> Option<f32> {
    if is_training_mode() {
        if MENU.shield_state == Shield::Infinite {
            if param_type == hash40("common") {
                if param_hash == hash40("shield_dec1") {
                    return Some(0.0);
                }
                if param_hash == hash40("shield_recovery1") {
                    return Some(999.0);
                }
                // doesn't work, somehow. This parameter isn't checked?
                if param_hash == hash40("shield_damage_mul") {
                    return Some(0.0);
                }
            }
        }
    }

    None
}

pub unsafe fn should_hold_shield(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    // We should hold shield if the state requires it
    if [Shield::Hold, Shield::Infinite].contains(&MENU.shield_state) {
        // If we are not mashing then we will always hold shield
        if MENU.mash_state == Mash::None {
            return true;
        }

        if !is_in_shieldstun(module_accessor) {
            return true;
        }

        // We will only drop shield if we are in shieldstun and our attack can be performed OOS
        if MENU.mash_state == Mash::Attack {
            if [Attack::NeutralB, Attack::SideB, Attack::DownB].contains(&MENU.mash_attack_state) {
                return false;
            }

            if MENU.mash_attack_state == Attack::Grab {
                return true;
            }
        }

        if MENU.mash_state == Mash::Spotdodge {
            return true;
        }
    }

    false
}

pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            if should_hold_shield(module_accessor) {
                return Some(true);
            }
        }
    }

    None
}

pub unsafe fn check_button_off(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            if should_hold_shield(module_accessor) {
                return Some(false);
            }
        }
    }

    None
}
