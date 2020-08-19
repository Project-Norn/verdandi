use crate::ast::AST;

struct Codegen {
    output: String,
}

pub fn generate(ast: AST) -> Result<String, String> {
    let mut codegen = Codegen::new();
    codegen.generate(ast)
}

impl Codegen {
    fn new() -> Self {
        Codegen {
            output: String::new(),
        }
    }

    fn generate(&mut self, ast: AST) -> Result<String, String> {
        self.gen(".intel_syntax noprefix");
        self.gen_function(ast)?;
        Ok(self.output.clone())
    }

    fn gen_function(&mut self, ast: AST) -> Result<(), String> {
        if let AST::Function { name, body } = ast {
            self.gen(&format!(".global {}", name));
            self.gen_label(name);
            self.gen_statement(*body)?;
            Ok(())
        } else {
            Err(format!("expected function, but got {:?}", ast))
        }
    }

    fn gen_statement(&mut self, ast: AST) -> Result<(), String> {
        match ast {
            AST::Return { value } => {
                self.gen_expression(*value)?;
                self.gen("  pop eax");
                self.gen("  ret");
                Ok(())
            }
            x => return Err(format!("unexpected node: {:?}", x)),
        }
    }

    fn gen_expression(&mut self, ast: AST) -> Result<(), String> {
        match ast {
            AST::Integer { value } => self.gen_integer(value),
            AST::Add { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_add();
            }
            AST::Sub { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_sub();
            }
            AST::Mul { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_mul();
            }
            AST::Div { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_div();
            }
            x => return Err(format!("unexpected node: {:?}", x)),
        }
        Ok(())
    }

    fn gen_label(&mut self, name: String) {
        self.gen(&format!("{}:", name));
    }

    fn gen_integer(&mut self, value: u32) {
        self.gen(&format!("  push {}", value));
    }

    fn gen_add(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  add eax, ecx");
        self.gen("  push eax");
    }

    fn gen_sub(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  sub eax, ecx");
        self.gen("  push eax");
    }

    fn gen_mul(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  imul ecx");
        self.gen("  push eax");
    }

    fn gen_div(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  xor edx, edx");
        self.gen("  idiv ecx");
        self.gen("  push eax");
    }

    fn gen(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push_str("\n");
    }
}
