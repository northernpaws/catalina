use crate::Machine;

/// Defines the parameters that can be set for a MIDI machine.
#[repr(u16)]
pub enum MIDIMachineParameter {
    Unknown = 0,

    /// Specifies the MIDI channel to send notes to.
    MIDIChannel = 1,
    /// Sends a bank change message.
    MIDIBankChange = 2,
    /// Sends a sub-bank change message.
    MIDISubBankChange = 3,
    /// Sends a program change message.
    MIDIPRogramChange = 4,
    /// Sends a pitch bend change message.
    MIDIPitchBend = 5,
    /// Sends an aftertouch change message.
    MIDIAftertouch = 6,
    /// Sends a mod wheel change message.
    MIDIModWheel = 7,
    /// Sends a breath control message.
    MIDIBreathControl = 8,
    // TODO: optional configurable CC parameters somehow?
}

/// The MIDI machine provides a means of playing out
/// MIDI notes to an external instrument or device.
pub struct MIDIMachine {}

impl MIDIMachine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Machine for MIDIMachine {}
