use crate::sandbox::new_ai::{Status, behavior_tree::{ExecReport, State}};

pub fn down_tick_selector (state_maybe: Option<State>,) -> ExecReport {
    let (child_index, child_state_maybe, reason_for_failure) =
        if let Some(state) = state_maybe {
            if let State::Selector {
                child_index,
                child_state_maybe,
                reason_for_failure,
            } = state
            {
                (
                    child_index,
                    if let Some(child_state) = child_state_maybe {
                        Some(*child_state)
                    } else {
                        None
                    },
                    reason_for_failure,
                )
            } else {
                return ExecReport::Status {
                    status: Status::Failure {
                        reason: format!("Excepted a Selector state, got {state:?}"),
                    },
                };
            }
        } else {
            (0, None, "".to_owned())
        };

    return ExecReport::TickChild {
        child_index,
        my_state: State::Selector {
            child_index,
            child_state_maybe: None,
            reason_for_failure,
        },
        child_state_maybe,
    };
}
