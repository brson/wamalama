pub mod types {
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum NumType {
        I32, I64, F32, F64,
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum VecType {
        V128,
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum RefType {
        FuncRef,
        ExternRef,
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum ValType {
        NumType(NumType),
        VecType(VecType),
        RefType(RefType),
    }

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct ResultType(Vec<ValType>);

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct FuncType {
        intype: ResultType,
        outtype: ResultType,
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct Limits {
        min: u32,
        max: Option<u32>,
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct MemType(Limits);

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct TableType(Limits, RefType);

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum Mut { Const, Var }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct GlobalType(Mut, ValType);

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum ExternType {
        Func(FuncType),
        Table(TableType),
        Mem(MemType),
        Global(GlobalType),
    }

    #[rustx::extension_trait::extension_trait]
    impl VecExternTypeExt for Vec<ExternType> {
        fn funcs(&self) -> Vec<FuncType> {
            self.iter().filter_map(|t| match t {
                ExternType::Func(t) => Some(t.clone()),
                _ => None,
            }).collect()
        }

        fn tables(&self) -> Vec<TableType> {
            self.iter().filter_map(|t| match t {
                ExternType::Table(t) => Some(t.clone()),
                _ => None,
            }).collect()
        }

        fn mems(&self) -> Vec<MemType> {
            self.iter().filter_map(|t| match t {
                ExternType::Mem(t) => Some(t.clone()),
                _ => None,
            }).collect()
        }

        fn globals(&self) -> Vec<GlobalType> {
            self.iter().filter_map(|t| match t {
                ExternType::Global(t) => Some(t.clone()),
                _ => None,
            }).collect()
        }
    }
}

pub mod instrs {
    pub enum Instr {
        I32Const(u32),
        I64Const(u64),
        F32Const(f32),
        F64Const(f64),
        I32Unop(IUnOp),
        I64Unop(IUnOp),
        F32Unop(FUnOp),
        F64Unop(FUnOp),
        I32ITestOp(ITestOp),
        I64ITestOp(ITestOp),
        I32IRelOp(IRelOp),
        I64IRelOp(IRelOp),
    }

    pub enum IUnOp {
        Clz, Ctz, Popcnt,
    }

    pub enum FUnOp {
        Abs, Neg, Sqrt, Ceil, Floor, Trunc, Nearest,
    }

    pub enum ITestOp {
        Eqz,
    }

    pub enum IRelOp {
        Eq, Ne, LtU, LtS, GtU, GtS, LeU, LeS, GeU, GeS,
    }
}
