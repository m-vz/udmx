// Documented in https://github.com/mirdej/udmx/blob/master/common/uDMX_cmds.h
pub enum Command {
    SetSingleChannel,
    SetChannelRange,
    StartBootloader,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::SetSingleChannel => 1,
            Command::SetChannelRange => 2,
            Command::StartBootloader => 0xf8,
        }
    }
}
