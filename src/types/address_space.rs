use super::kind::*;

#[derive(Debug)]
pub enum AddressSpace { Identifier(String), Global, Local, Private, Constant }

impl Identifier for AddressSpace {
    fn kind(&self) -> Kind {
        return Kind::AddrSpace;
    }

    fn name(&self) -> &str {
        return match self {
            AddressSpace::Identifier(s) => s,
            _ => unreachable!()
        }
    }
}
