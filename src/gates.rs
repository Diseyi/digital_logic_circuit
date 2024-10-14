
pub trait Logic {
    fn logic(&self) -> bool;
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
    fn logic(&self) -> bool {
        self.a && self.b
    }
}

impl Logic for OR {
    fn logic(&self) -> bool {
        self.a || self.b
    }
}

impl Logic for NOT {
    fn logic(&self) -> bool {
        !self.a
    }
}

impl Logic for NAND {
    fn logic(&self) -> bool {
        !self.a || !self.b
    }
}

impl Logic for NOR {
    fn logic(&self) -> bool {
        !self.a && !self.b
    }
}

impl Logic for XOR {
    fn logic(&self) -> bool {
        self.a ^ self.b
    }
}

impl Logic for XNOR {
    fn logic(&self) -> bool {
       !self.a ^ self.b
    }
}

// TODO 
// impl Fn<()> for AND {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         self.logic()
//     }
// }
