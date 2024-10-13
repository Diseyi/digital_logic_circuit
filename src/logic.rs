
pub trait Logic {
    fn logic(&self) -> u8;
}

pub struct AND {
    pub a: bool,
    pub b: bool,
}
impl Logic for AND {
    fn logic(&self) -> u8 {
        (self.a && self.b) as u8
    }
}

pub struct OR {
    pub a: bool,
    pub b: bool,
}
impl Logic for OR {
    fn logic(&self) -> u8 {
        (self.a || self.b) as u8
    }
}

pub struct NOT {
    pub a: bool,
}
impl Logic for NOT {
    fn logic(&self) -> u8 {
        !self.a as u8
    }
}

// TODO 
// impl Fn<()> for AND {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         self.logic()
//     }
// }
