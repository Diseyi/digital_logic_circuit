
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert!(AND { a: true, b: true }.logic());
        assert!(!AND { a: true, b: false }.logic());
        assert!(!AND { a: false, b: true }.logic());
        assert!(!AND { a: false, b: false }.logic());
    }

    #[test]
    fn test_or() {
        assert!(OR { a: true, b: true }.logic());
        assert!(OR { a: true, b: false  }.logic());
        assert!(OR { a: false, b: true}.logic());
        assert!(!OR { a: false, b: false }.logic());
    }

    #[test]
    fn test_not() {
        assert!(!NOT { a: true }.logic());
        assert!(NOT { a: false }.logic());
    }

    #[test]
    fn test_nand() {
        assert!(!NAND { a: true, b: true }.logic());
        assert!(NAND { a: true, b: false  }.logic());
        assert!(NAND { a: false, b: true}.logic());
        assert!(NAND { a: false, b: false }.logic());
    }

    #[test]
    fn test_nor() {
        assert!(!NOR { a: true, b: true }.logic());
        assert!(!NOR { a: true, b: false  }.logic());
        assert!(!NOR { a: false, b: true }.logic());
        assert!(NOR {a: false, b: false}.logic());
    }

    #[test]
    fn test_xor() {
        assert!(!XOR { a: true, b: true }.logic());
        assert!(XOR { a: true, b: false  }.logic());
        assert!(XOR { a: false, b: true }.logic());
        assert!(!XOR { a: false, b: false }.logic());
    }

    #[test]
    fn test_xnor() {
        assert!(XNOR { a: true, b: true }.logic());
        assert!(!XNOR { a: true, b: false  }.logic());
        assert!(!XNOR { a: false, b: true}.logic());
        assert!(XNOR { a: false, b: false }.logic());
    }
}
