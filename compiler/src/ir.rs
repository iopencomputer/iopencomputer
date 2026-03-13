#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instrs: Vec<Instr>,
    pub term: Terminator,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Add { dest: String, lhs: String, rhs: String },
    Sub { dest: String, lhs: String, rhs: String },
    Mul { dest: String, lhs: String, rhs: String },
    ICmp { dest: String, pred: String, lhs: String, rhs: String },
    Call { dest: Option<String>, func: String, args: Vec<String> },
    Alloca { dest: String },
    Store { value: String, ptr: String },
    Load { dest: String, ptr: String },
    Phi { dest: String, incomings: Vec<(String, String)> },
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret { value: String },
    Br { dest: String },
    CondBr { cond: String, then_dest: String, else_dest: String },
}

impl ToString for Module {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for func in &self.functions {
            let params = func
                .params
                .iter()
                .map(|p| format!("i32 {}", p))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("define i32 @{}({}) {{\n", func.name, params));
            for bb in &func.blocks {
                out.push_str(&format!("{}:\n", bb.name));
                for instr in &bb.instrs {
                    out.push_str(&format!("  {}\n", instr.to_string()));
                }
                out.push_str(&format!("  {}\n", bb.term.to_string()));
                out.push('\n');
            }
            out.push_str("}\n\n");
        }
        out
    }
}

impl ToString for Instr {
    fn to_string(&self) -> String {
        match self {
            Instr::Add { dest, lhs, rhs } => format!("{} = add i32 {}, {}", dest, lhs, rhs),
            Instr::Sub { dest, lhs, rhs } => format!("{} = sub i32 {}, {}", dest, lhs, rhs),
            Instr::Mul { dest, lhs, rhs } => format!("{} = mul i32 {}, {}", dest, lhs, rhs),
            Instr::ICmp {
                dest,
                pred,
                lhs,
                rhs,
            } => format!("{} = icmp {} i32 {}, {}", dest, pred, lhs, rhs),
            Instr::Call { dest, func, args } => {
                let args = args
                    .iter()
                    .map(|a| format!("i32 {}", a))
                    .collect::<Vec<_>>()
                    .join(", ");
                match dest {
                    Some(d) => format!("{} = call i32 @{}({})", d, func, args),
                    None => format!("call i32 @{}({})", func, args),
                }
            }
            Instr::Alloca { dest } => format!("{} = alloca i32", dest),
            Instr::Store { value, ptr } => format!("store i32 {}, i32* {}", value, ptr),
            Instr::Load { dest, ptr } => format!("{} = load i32, i32* {}", dest, ptr),
            Instr::Phi { dest, incomings } => {
                let list = incomings
                    .iter()
                    .map(|(v, b)| format!("[ {}, %{} ]", v, b))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} = phi i32 {}", dest, list)
            }
        }
    }
}

impl ToString for Terminator {
    fn to_string(&self) -> String {
        match self {
            Terminator::Ret { value } => format!("ret i32 {}", value),
            Terminator::Br { dest } => format!("br label %{}", dest),
            Terminator::CondBr {
                cond,
                then_dest,
                else_dest,
            } => format!(
                "br i1 {}, label %{}, label %{}",
                cond, then_dest, else_dest
            ),
        }
    }
}
