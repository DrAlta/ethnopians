/// do to node returning running if they still have children to tick or if their
/// child returned running implments the type of selector/sequence that starts at the
/// first child every time is hard
/// So add a 4th 'Ticking{state: State}' `Status`.
///
/// and rename `Running` to `Waiting` to make it more clear that it's for when the
/// node is waiting on the external world
#[derive(Debug, Clone, PartialEq)]
pub enum Status<State> {
    Success,
    Failure { reason: String },
    //    Ticking{child: usize, my_state: State, child_state_maybe: Option<State>},
    // Tradition BT Running is Waiting{state: State}
    Waiting { state: State },
}
