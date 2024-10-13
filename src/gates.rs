
pub trait Logic {
    fn logic(&self) -> u8;
}

pub struct AND {
    pub a: bool,
    pub b: bool,
}

pub struct OR {
    pub a: bool,
    pub b: bool,
}

pub struct NOT {
    pub a: bool,
}

pub struct NAND {
    pub a: bool,
    pub b: bool,
}

pub struct NOR {
    pub a: bool,
    pub b: bool,
}

pub struct XOR {
    pub a: bool,
    pub b: bool,
}

pub struct XNOR {
    pub a: bool,
    pub b: bool,
}

impl Logic for AND {
    fn logic(&self) -> u8 {
        (self.a && self.b) as u8
    }
}

impl Logic for OR {
    fn logic(&self) -> u8 {
        (self.a || self.b) as u8
    }
}

impl Logic for NOT {
    fn logic(&self) -> u8 {
        !self.a as u8
    }
}

impl Logic for NAND {
    fn logic(&self) -> u8 {
        (!self.a || !self.b) as u8
    }
}

impl Logic for NOR {
    fn logic(&self) -> u8 {
        (!self.a && !self.b) as u8
    }
}

impl Logic for XOR {
    fn logic(&self) -> u8 {
        (self.a ^ self.b) as u8
    }
}

impl Logic for XNOR {
    fn logic(&self) -> u8 {
       !(self.a ^ self.b) as u8
    }
}

// TODO 
// impl Fn<()> for AND {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         self.logic()
//     }
// }
