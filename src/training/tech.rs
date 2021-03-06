use crate::common::consts::*;
use crate::common::*;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;

pub unsafe fn init_settings(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    status_kind: i32,
) -> Option<()> {
    if is_training_mode() && is_operation_cpu(module_accessor) {
        if status_kind == FIGHTER_STATUS_KIND_DOWN {
            match MENU.tech_state {
                TechOption::Random => {
                    let random_statuses = vec![
                        *FIGHTER_STATUS_KIND_DOWN,
                        *FIGHTER_STATUS_KIND_PASSIVE,
                        *FIGHTER_STATUS_KIND_PASSIVE_FB,
                    ];

                    let random_status_index =
                        app::sv_math::rand(hash40("fighter"), random_statuses.len() as i32)
                            as usize;
                    if random_statuses[random_status_index] != FIGHTER_STATUS_KIND_DOWN {
                        StatusModule::change_status_request_from_script(
                            module_accessor,
                            random_statuses[random_status_index],
                            true,
                        );
                        return Some(());
                    }
                }
                TechOption::InPlace => {
                    StatusModule::change_status_request_from_script(
                        module_accessor,
                        *FIGHTER_STATUS_KIND_PASSIVE,
                        true,
                    );
                    return Some(());
                }
                TechOption::Roll => {
                    StatusModule::change_status_request_from_script(
                        module_accessor,
                        *FIGHTER_STATUS_KIND_PASSIVE_FB,
                        true,
                    );
                    return Some(());
                }
                _ => (),
            }
        }
    }

    None
}

pub unsafe fn should_perform_defensive_option(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    prev_status: i32,
    status: i32,
) -> bool {
    ([
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
    ]
    .contains(&prev_status)
        || [
            *FIGHTER_STATUS_KIND_DOWN_STAND,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
        ]
        .contains(&status)
    )
        && (
            WorkModule::is_enable_transition_term(
                module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
            )
            ||
            CancelModule::is_enable_cancel(module_accessor)
        )
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    _category: i32,
    flag: &mut i32,
) {
    if MENU.tech_state != TechOption::None && is_training_mode() && is_operation_cpu(module_accessor) {
        let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
        let status = StatusModule::status_kind(module_accessor) as i32;
        if [
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
        ]
        .contains(&status)
        {
            let random_statuses = vec![
                *FIGHTER_STATUS_KIND_DOWN_STAND,
                *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
                *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
            ];

            let random_status_index =
                app::sv_math::rand(hash40("fighter"), random_statuses.len() as i32) as usize;
            StatusModule::change_status_request_from_script(
                module_accessor,
                random_statuses[random_status_index],
                true,
            );
        } else if should_perform_defensive_option(module_accessor, prev_status, status) {
            perform_defensive_option(module_accessor, flag);
        }
    }
}

pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32,
) -> Option<bool> {
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
            let status = StatusModule::status_kind(module_accessor) as i32;
            if MENU.defensive_state == Defensive::Shield
                && should_perform_defensive_option(module_accessor, prev_status, status)
            {
                return Some(true);
            }
        }
    }

    None
}

pub unsafe fn change_motion(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64,
) -> Option<u64> {
    if MENU.tech_state != TechOption::None && is_training_mode() && is_operation_cpu(module_accessor) {
        if [hash40("passive_stand_f"), hash40("passive_stand_b")].contains(&motion_kind) {
            if app::sv_math::rand(hash40("fighter"), 2) != 0 {
                return Some(hash40("passive_stand_f"));
            } else {
                return Some(hash40("passive_stand_b"));
            }
        } else if [hash40("down_forward_u"), hash40("down_back_u")].contains(&motion_kind) {
            if app::sv_math::rand(hash40("fighter"), 2) != 0 {
                return Some(hash40("down_forward_u"));
            } else {
                return Some(hash40("down_back_u"));
            }
        } else if [hash40("down_forward_d"), hash40("down_back_d")].contains(&motion_kind) {
            if app::sv_math::rand(hash40("fighter"), 2) != 0 {
                return Some(hash40("down_forward_d"));
            } else {
                return Some(hash40("down_back_d"));
            }
        }
    }

    None
}
