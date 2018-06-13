
/// trait that marks primitives that "numeric" columns can represent
pub trait Numeric: Sized + Copy + Clone {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

impl Numeric for i8 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i8()
    }
}

impl Numeric for i16 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i16()
    }
}

impl Numeric for i32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i32()
    }
}

impl Numeric for i64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_i64()
    }
}

impl Numeric for u8 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u8()
    }
}

impl Numeric for u16 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u16()
    }
}

impl Numeric for u32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u32()
    }
}

impl Numeric for u64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_u64()
    }
}

impl Numeric for f32 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_float().map(|f| f as f32)
    }
}

impl Numeric for f64 {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.parse_float()
    }
}
