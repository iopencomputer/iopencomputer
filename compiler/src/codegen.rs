use crate::ast::{BinOp, Expr, Function, Program, Stmt};
use crate::ir::{BasicBlock, Instr, Module, Terminator};
use anyhow::Result;
use std::collections::HashMap;

pub fn lower_to_ir(program: Program) -> Result<Module> {
    let mut functions = Vec::new();
    for f in &program.functions {
        functions.push(lower_function(f)?);
    }
    Ok(Module { functions })
}

fn lower_function(func: &Function) -> Result<crate::ir::Function> {
    let mut builder = FunctionBuilder::new();
    let entry = builder.new_block("entry");
    builder.set_current(entry);

    let value = builder.emit_expr(&func.body)?;
    builder.set_terminator(Terminator::Ret { value });

    Ok(crate::ir::Function {
        name: func.name.clone(),
        params: func
            .params
            .iter()
            .map(|p| format!("%{}", p.name))
            .collect(),
        blocks: builder.blocks,
    })
}

struct FunctionBuilder {
    blocks: Vec<BasicBlock>,
    current: usize,
    temp: usize,
    label: usize,
    locals: HashMap<String, String>,
}

impl FunctionBuilder {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            current: 0,
            temp: 0,
            label: 0,
            locals: HashMap::new(),
        }
    }

    fn new_block(&mut self, name: &str) -> usize {
        let idx = self.blocks.len();
        self.blocks.push(BasicBlock {
            name: name.to_string(),
            instrs: Vec::new(),
            term: Terminator::Ret {
                value: "0".to_string(),
            },
        });
        idx
    }

    fn set_current(&mut self, idx: usize) {
        self.current = idx;
    }

    fn current_block_mut(&mut self) -> &mut BasicBlock {
        &mut self.blocks[self.current]
    }

    fn set_terminator(&mut self, term: Terminator) {
        self.blocks[self.current].term = term;
    }

    fn new_temp(&mut self) -> String {
        self.temp += 1;
        format!("%t{}", self.temp)
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label += 1;
        format!("{}{}", prefix, self.label)
    }

    fn emit_expr(&mut self, expr: &Expr) -> Result<String> {
        match expr {
            Expr::Int(v) => Ok(v.to_string()),
            Expr::Bool(b) => Ok(if *b { "true".to_string() } else { "false".to_string() }),
            Expr::Ident(name) => {
                if let Some(ptr) = self.locals.get(name).cloned() {
                    let dest = self.new_temp();
                    self.current_block_mut()
                        .instrs
                        .push(Instr::Load { dest: dest.clone(), ptr });
                    Ok(dest)
                } else {
                    Ok(format!("%{}", name))
                }
            }
            Expr::Binary { op, lhs, rhs } => {
                let l = self.emit_expr(lhs)?;
                let r = self.emit_expr(rhs)?;
                let dest = self.new_temp();
                match op {
                    BinOp::Add => self.current_block_mut().instrs.push(Instr::Add {
                        dest: dest.clone(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Sub => self.current_block_mut().instrs.push(Instr::Sub {
                        dest: dest.clone(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Mul => self.current_block_mut().instrs.push(Instr::Mul {
                        dest: dest.clone(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Lt => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "slt".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Le => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "sle".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Gt => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "sgt".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Ge => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "sge".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Eq => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "eq".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                    BinOp::Ne => self.current_block_mut().instrs.push(Instr::ICmp {
                        dest: dest.clone(),
                        pred: "ne".to_string(),
                        lhs: l,
                        rhs: r,
                    }),
                }
                Ok(dest)
            }
            Expr::Call { name, args } => {
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.emit_expr(a)?);
                }
                let dest = self.new_temp();
                self.current_block_mut().instrs.push(Instr::Call {
                    dest: Some(dest.clone()),
                    func: name.clone(),
                    args: arg_vals,
                });
                Ok(dest)
            }
            Expr::If {
                cond,
                then_expr,
                else_expr,
            } => {
                let cond_val = self.emit_expr(cond)?;
                let then_name = self.new_label("then");
                let else_name = self.new_label("else");
                let merge_name = self.new_label("merge");

                let then_idx = self.new_block(&then_name);
                let else_idx = self.new_block(&else_name);
                let merge_idx = self.new_block(&merge_name);

                self.set_terminator(Terminator::CondBr {
                    cond: cond_val,
                    then_dest: then_name.clone(),
                    else_dest: else_name.clone(),
                });

                self.set_current(then_idx);
                let then_val = self.emit_expr(then_expr)?;
                self.set_terminator(Terminator::Br {
                    dest: merge_name.clone(),
                });

                self.set_current(else_idx);
                let else_val = self.emit_expr(else_expr)?;
                self.set_terminator(Terminator::Br {
                    dest: merge_name.clone(),
                });

                self.set_current(merge_idx);
                let dest = self.new_temp();
                self.current_block_mut().instrs.push(Instr::Phi {
                    dest: dest.clone(),
                    incomings: vec![
                        (then_val, then_name),
                        (else_val, else_name),
                    ],
                });
                Ok(dest)
            }
            Expr::Block { stmts, expr } => {
                for stmt in stmts {
                    self.emit_stmt(stmt)?;
                }
                self.emit_expr(expr)
            }
        }
    }

    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let ptr = format!("%{}.addr", name);
                if !self.locals.contains_key(name) {
                    self.blocks[0]
                        .instrs
                        .insert(0, Instr::Alloca { dest: ptr.clone() });
                    self.locals.insert(name.clone(), ptr.clone());
                }
                let val = self.emit_expr(value)?;
                self.current_block_mut()
                    .instrs
                    .push(Instr::Store { value: val, ptr });
                Ok(())
            }
        }
    }
}
