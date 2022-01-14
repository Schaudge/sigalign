// Alignment algorithms
use crate::core::{
	Penalties, PRECISION_SCALE, Cutoff, MinPenaltyForPattern,
	ReferenceAlignmentResult, RecordAlignmentResult, AlignmentResult, AlignmentPosition, AlignmentOperation, AlignmentType,
    Sequence,
    ReferenceInterface, PatternLocation,
    AlignerInterface,
};

mod common_steps;
pub use common_steps::{Extension, AlignmentHashSet, WaveFront, WaveEndPoint, WaveFrontScore, Components, Component, BackTraceMarker, calculate_spare_penalty_from_determinant};

mod semi_global;
mod local;
pub use local::local_alignment_algorithm;
pub use semi_global::semi_global_alignment_algorithm;