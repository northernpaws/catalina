/// For computational efficiancy, a parameter ID is represented
/// as a number to allow faster lookup then string names.
///
/// A parameter name can be resolved by feeding the parameter ID
/// to the parameter names string table.
pub type ParameterID = usize;

/// Indicates the type of a parameter value.
#[repr(u8)]
pub enum ParameterValueKind {
    Bool,
}

/// Represents the value of a parameter.
pub enum ParameterValue {
    Bool(bool),
}

impl ParameterValue {
    /// Returns the type of the parameter value.
    pub fn kind(&self) -> ParameterValueKind {
        match self {
            ParameterValue::Bool(_) => ParameterValueKind::Bool,
        }
    }
}

/// Indicates the domain of the parameter, used for routing.
#[repr(u8)]
pub enum ParameterDomain {
    /// Indicates that the parameter control some function
    /// of the track sequencing or playback that it's in.
    Track,
}

/// Defines details about a parameter.
pub struct Parameter {
    /// Indicates the domain of the parameter.
    domain: ParameterDomain,
    /// Indicates the kind of the values of the parameter.
    value_kind: ParameterValueKind,
}
