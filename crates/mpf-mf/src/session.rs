//! The node-state helpers of `dlls/mf/session.c`: the object state that a media event moves a
//! source, stream or sink to, and the checks that the source and output nodes are all in one state.
//! The C walks the session's lists of media sources and topology nodes. These functions take the
//! node states as slices, so the caller collects them first.

const ME_SOURCE_STARTED: u32 = 201;
const ME_STREAM_STARTED: u32 = 202;
const ME_SOURCE_SEEKED: u32 = 203;
const ME_STREAM_SEEKED: u32 = 204;
const ME_SOURCE_STOPPED: u32 = 207;
const ME_STREAM_STOPPED: u32 = 208;
const ME_SOURCE_PAUSED: u32 = 209;
const ME_STREAM_PAUSED: u32 = 210;
const ME_STREAM_SINK_STARTED: u32 = 301;
const ME_STREAM_SINK_STOPPED: u32 = 302;
const ME_STREAM_SINK_PAUSED: u32 = 303;
const ME_STREAM_SINK_PREROLLED: u32 = 307;

/// `enum object_state`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectState {
    Stopped,
    Started,
    Paused,
    Prerolled,
    Invalid,
}

/// `enum command_state`: how far the session has got with its current command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandState {
    Complete,
    Submitted,
    RestartingSources,
    StartingSources,
    PrerollingSinks,
    StartingSinks,
    PausingSinks,
    PausingSources,
    StoppingSinks,
    StoppingSources,
    ClosingSinks,
    ClosingSources,
    FinalizingSinks,
}

/// `MF_TOPOLOGY_TYPE`: the kinds of topology node that the state checks tell apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Output,
    SourceStream,
    Transform,
    Tee,
}

/// The object state that the media event `event` moves its object to. A stopped sink event during
/// the start of a topology is `Invalid`, because the clock is not running yet, so no stop happened.
/// An unknown event is `Invalid`.
pub fn object_state_for_event(event: u32, command: CommandState) -> ObjectState {
    match event {
        ME_SOURCE_STARTED
        | ME_STREAM_STARTED
        | ME_SOURCE_SEEKED
        | ME_STREAM_SEEKED
        | ME_STREAM_SINK_STARTED => ObjectState::Started,
        ME_SOURCE_PAUSED | ME_STREAM_PAUSED | ME_STREAM_SINK_PAUSED => ObjectState::Paused,
        ME_STREAM_SINK_STOPPED
            if matches!(
                command,
                CommandState::PrerollingSinks | CommandState::StartingSinks
            ) =>
        {
            ObjectState::Invalid
        }
        ME_SOURCE_STOPPED | ME_STREAM_STOPPED | ME_STREAM_SINK_STOPPED => ObjectState::Stopped,
        ME_STREAM_SINK_PREROLLED => ObjectState::Prerolled,
        _ => ObjectState::Invalid,
    }
}

/// Whether every source and every `SourceStream` node is in `state`. The other node types are not
/// checked.
pub fn source_nodes_in_state(
    sources: &[ObjectState],
    nodes: &[(NodeType, ObjectState)],
    state: ObjectState,
) -> bool {
    sources.iter().all(|&source| source == state)
        && nodes_of_type_in_state(nodes, NodeType::SourceStream, state)
}

/// Whether every `Output` node is in `state`. The other node types are not checked.
pub fn output_nodes_in_state(nodes: &[(NodeType, ObjectState)], state: ObjectState) -> bool {
    nodes_of_type_in_state(nodes, NodeType::Output, state)
}

fn nodes_of_type_in_state(
    nodes: &[(NodeType, ObjectState)],
    node_type: NodeType,
    state: ObjectState,
) -> bool {
    nodes
        .iter()
        .all(|&(kind, node_state)| kind != node_type || node_state == state)
}
