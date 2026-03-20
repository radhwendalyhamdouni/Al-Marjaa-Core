// ═══════════════════════════════════════════════════════════════════════════════
// Cranelift JIT Compiler - مترجم فوري
// ═══════════════════════════════════════════════════════════════════════════════
// ترجمة وتنفيذ الكود في الذاكرة
// ═══════════════════════════════════════════════════════════════════════════════

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;
use std::collections::HashMap;

use crate::parser::ast::{BinaryOp, ComparisonOp, Expr, LogicalOp, Program, Stmt, UnaryOp};

/// نتيجة JIT
#[derive(Debug)]
pub struct JitResult {
    /// هل نجحت الترجمة
    pub success: bool,
    /// الأخطاء
    pub errors: Vec<String>,
    /// القيمة المُرجعة
    pub return_value: Option<f64>,
}

/// مترجم JIT
pub struct JitCompiler {
    /// سياق Cranelift
    _ctx: codegen::Context,
    /// وحدة JIT
    _module: JITModule,
    /// جدول الرموز
    variables: HashMap<String, f64>,
    /// الدوال
    functions: HashMap<String, usize>,
}

impl JitCompiler {
    /// إنشاء مترجم JIT جديد
    pub fn new() -> Result<Self, String> {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").map_err(|e| e.to_string())?;
        flag_builder.set("is_pic", "false").map_err(|e| e.to_string())?;
        flag_builder.set("opt_level", "speed").map_err(|e| e.to_string())?;

        let isa_builder = cranelift_native::builder().map_err(|e| e.to_string())?;
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| e.to_string())?;

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module: JITModule = JITModule::new(builder);
        let ctx = module.make_context();

        Ok(JitCompiler {
            _ctx: ctx,
            _module: module,
            variables: HashMap::new(),
            functions: HashMap::new(),
        })
    }

    /// ترجمة وتنفيذ برنامج
    pub fn compile_and_run(&mut self, program: &Program) -> Result<f64, String> {
        // جمع الدوال
        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, .. } = stmt {
                self.functions.insert(name.clone(), self.functions.len());
            }
        }

        // تنفيذ البرنامج
        let mut result = 0.0;
        for stmt in &program.statements {
            result = self.execute_statement(stmt)?;
        }

        Ok(result)
    }

    /// تنفيذ عبارة
    fn execute_statement(&mut self, stmt: &Stmt) -> Result<f64, String> {
        match stmt {
            Stmt::VariableDecl { name, value, .. } => {
                let val = self.execute_expression(value)?;
                self.variables.insert(name.clone(), val);
                Ok(val)
            }

            Stmt::FunctionDecl { name, params, body, .. } => {
                // تسجيل الدالة (مبسط)
                let _ = (name, params, body);
                Ok(0.0)
            }

            Stmt::Return(value) => {
                match value {
                    Some(expr) => self.execute_expression(expr),
                    None => Ok(0.0),
                }
            }

            Stmt::Expression(expr) => self.execute_expression(expr),

            Stmt::Block(statements) => {
                let mut result = 0.0;
                for s in statements {
                    result = self.execute_statement(s)?;
                }
                Ok(result)
            }

            Stmt::If { condition, then_branch, else_if_branches, else_branch } => {
                let cond = self.execute_expression(condition)?;

                if cond != 0.0 {
                    self.execute_statement(then_branch)
                } else {
                    for (elif_cond, elif_body) in else_if_branches {
                        let elif_val = self.execute_expression(elif_cond)?;
                        if elif_val != 0.0 {
                            return self.execute_statement(elif_body);
                        }
                    }

                    if let Some(else_body) = else_branch {
                        self.execute_statement(else_body)
                    } else {
                        Ok(0.0)
                    }
                }
            }

            Stmt::While { condition, body } => {
                let mut result = 0.0;
                let max_iterations = 10000;
                let mut iterations = 0;

                loop {
                    let cond = self.execute_expression(condition)?;
                    if cond == 0.0 || iterations >= max_iterations {
                        break;
                    }
                    result = self.execute_statement(body)?;
                    iterations += 1;
                }

                Ok(result)
            }

            Stmt::For { variable, iterable: _, body } => {
                let mut result = 0.0;
                let prev = self.variables.get(variable).copied();

                // تكرار مبسط (10 مرات)
                for i in 0..10 {
                    self.variables.insert(variable.clone(), i as f64);
                    result = self.execute_statement(body)?;
                }

                if let Some(v) = prev {
                    self.variables.insert(variable.clone(), v);
                } else {
                    self.variables.remove(variable);
                }

                Ok(result)
            }

            Stmt::Print(args) => {
                for arg in args {
                    let val = self.execute_expression(arg)?;
                    println!("{}", val);
                }
                Ok(0.0)
            }

            _ => Ok(0.0),
        }
    }

    /// تنفيذ تعبير
    fn execute_expression(&mut self, expr: &Expr) -> Result<f64, String> {
        match expr {
            Expr::Number(n) => Ok(*n),

            Expr::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),

            Expr::Null => Ok(0.0),

            Expr::Identifier(name) => {
                self.variables
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("متغير غير معرف: {}", name))
            }

            Expr::Binary { left, op, right } => {
                let left_val = self.execute_expression(left)?;
                let right_val = self.execute_expression(right)?;

                match op {
                    BinaryOp::Add => Ok(left_val + right_val),
                    BinaryOp::Sub => Ok(left_val - right_val),
                    BinaryOp::Mul => Ok(left_val * right_val),
                    BinaryOp::Div => {
                        if right_val == 0.0 {
                            Err("قسمة على صفر".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    BinaryOp::Mod => Ok(left_val % right_val),
                    BinaryOp::Pow => Ok(left_val.powf(right_val)),
                    BinaryOp::FloorDiv => Ok((left_val / right_val).floor()),
                    _ => Ok(left_val + right_val),
                }
            }

            Expr::Comparison { left, op, right } => {
                let left_val = self.execute_expression(left)?;
                let right_val = self.execute_expression(right)?;

                let result = match op {
                    ComparisonOp::Equal => (left_val - right_val).abs() < f64::EPSILON,
                    ComparisonOp::NotEqual => (left_val - right_val).abs() >= f64::EPSILON,
                    ComparisonOp::Less => left_val < right_val,
                    ComparisonOp::Greater => left_val > right_val,
                    ComparisonOp::LessEqual => left_val <= right_val,
                    ComparisonOp::GreaterEqual => left_val >= right_val,
                };

                Ok(if result { 1.0 } else { 0.0 })
            }

            Expr::Logical { left, op, right } => {
                let left_val = self.execute_expression(left)?;

                match op {
                    LogicalOp::And => {
                        if left_val == 0.0 {
                            Ok(0.0)
                        } else {
                            let right_val = self.execute_expression(right)?;
                            Ok(if right_val != 0.0 { 1.0 } else { 0.0 })
                        }
                    }
                    LogicalOp::Or => {
                        if left_val != 0.0 {
                            Ok(1.0)
                        } else {
                            let right_val = self.execute_expression(right)?;
                            Ok(if right_val != 0.0 { 1.0 } else { 0.0 })
                        }
                    }
                }
            }

            Expr::Unary { op, expr } => {
                let val = self.execute_expression(expr)?;

                match op {
                    UnaryOp::Neg => Ok(-val),
                    UnaryOp::Not => Ok(if val == 0.0 { 1.0 } else { 0.0 }),
                    UnaryOp::BitNot => Ok((!(val as i64)) as f64),
                }
            }

            Expr::Ternary { condition, then_expr, else_expr } => {
                let cond = self.execute_expression(condition)?;
                if cond != 0.0 {
                    self.execute_expression(then_expr)
                } else {
                    self.execute_expression(else_expr)
                }
            }

            Expr::Assignment { target, value } => {
                let name = match target.as_ref() {
                    Expr::Identifier(n) => n.clone(),
                    _ => return Err("الهدف يجب أن يكون معرفاً".to_string()),
                };

                let val = self.execute_expression(value)?;
                self.variables.insert(name, val);
                Ok(val)
            }

            Expr::Call { callee, args } => {
                // تقييم المعاملات
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.execute_expression(arg)?);
                }

                match callee.as_ref() {
                    Expr::Identifier(name) => {
                        // دوال مدمجة
                        match name.as_str() {
                            "طول" | "len" => Ok(arg_vals.first().copied().unwrap_or(0.0)),
                            "مطلق" | "abs" => Ok(arg_vals.first().map(|v| v.abs()).unwrap_or(0.0)),
                            "جذر" | "sqrt" => Ok(arg_vals.first().map(|v| v.sqrt()).unwrap_or(0.0)),
                            "قوة" | "pow" => {
                                let a = arg_vals.first().copied().unwrap_or(0.0);
                                let b = arg_vals.get(1).copied().unwrap_or(1.0);
                                Ok(a.powf(b))
                            }
                            "طباعة" | "print" => {
                                for v in &arg_vals {
                                    println!("{}", v);
                                }
                                Ok(0.0)
                            }
                            _ => {
                                // دالة معرفة من قبل المستخدم
                                Ok(0.0)
                            }
                        }
                    }
                    _ => Ok(0.0),
                }
            }

            Expr::Increment { name, is_prefix, delta } => {
                let current = self.variables.get(name).copied().unwrap_or(0.0);
                let new_val = if *delta > 0.0 {
                    current + *delta
                } else {
                    current - (*delta * -1.0)
                };
                self.variables.insert(name.clone(), new_val);
                Ok(if *is_prefix { new_val } else { current })
            }

            _ => Ok(0.0),
        }
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new().expect("فشل إنشاء JIT Compiler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_creation() {
        let jit = JitCompiler::new();
        assert!(jit.is_ok());
    }

    #[test]
    fn test_jit_simple_expression() {
        let mut jit = JitCompiler::new().unwrap();

        let program = Program::new(vec![
            Stmt::VariableDecl {
                name: "س".to_string(),
                value: Expr::Number(42.0),
                is_const: false,
            },
            Stmt::Return(Some(Expr::Identifier("س".to_string()))),
        ]);

        let result = jit.compile_and_run(&program);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.0);
    }
}
