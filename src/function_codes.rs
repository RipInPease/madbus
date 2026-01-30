pub enum CMDCode {
    /// Function code 0x01
    ReadCoils{
        start: u16,
        count: u16,
    },

    /// Function code 0x02
    ReadDI{
        start: u16,
        count: u16,
    },

    /// Function code 0x03
    ReadHolding{
        start: u16,
        count: u16,
    },

    /// Function code 0x04
    ReadInput{
        start: u16,
        count: u16,
    }
}