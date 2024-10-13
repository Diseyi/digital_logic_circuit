
pub trait Logic {
    fn logic(&self) -> u8;
}

pub struct AND {
    pub a: bool,
    pub b: bool,
}
impl Logic for AND {
    fn logic(&self) -> u8 {
        if self.a && self.b  {
            1
        } else {
            0
        }
    }
}

// TODO 
// impl Fn<()> for AND {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         self.logic()
//     }
// }
