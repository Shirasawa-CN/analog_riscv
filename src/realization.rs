/*
作者:Shirasawa-CN(Cpointerz)
贡献者:
创建时间:2022/8/24
 */



use tracing::error;

//RISC-V有六种基本指令格式：
//
//     R 类型指令,用于寄存器-寄存器操作
//     I 型指令，用于短立即数和访存 load 操作
//     S 型指令，用于访存 store 操作
//     B 类型指令，用于条件跳转操作
//     U 型指令，用于长立即数
//     J 型指令，用于无条件跳转
enum Type {
    R,
    I,
    I_LI, //有符号和半字型I类型
    S,
    B,
    U_LUI,   //加载高位立即数
    U_AUIPC, //立即数地址添加到寄存器中
    J,
    Error,
}

//定义RISC-V的数据结构
struct RV {
    //定义寄存器
    registers: [i64; 64],
    position_in_memory: String,
    //定义记录内存位置
    memory: [u128; 0x1000],
}

impl Default for RV {
    fn default() -> Self {
        Self {
            registers: [0; 64],
            position_in_memory: String::new(),
            memory: [0; 0x1000],
        }
    }
}

impl RV {
    //运行
    pub fn binary_auto_run(&mut self, value: String) {
        loop {
            self.read_position_in_memory(&value);
            let opcode = self.read_opcode();
            let riscv_type = Self::type_opcode(opcode);
            let code = String::from(opcode);

            match riscv_type {
                Type::R => Self::binary_run_R(code),
                Type::I => todo!("我是傻逼"),
                Type::I_LI => todo!("我是傻逼"),
                Type::S => todo!("我是傻逼"),
                Type::B => todo!("我是傻逼"),
                Type::U_LUI => todo!("我是傻逼"),
                Type::U_AUIPC => todo!("我是傻逼"),
                Type::J => todo!("我是傻逼"),
                Type::Error => error!("找不到该指令"),
                _ => error!("找不到该指令"),
            }
        }
    }
    //提取出opcode
    fn read_opcode(&self) -> &str {
        let p = &self.position_in_memory;
        let result = &p[25..31];
        result
    }
    //读取指令
    fn read_position_in_memory(&mut self, p: &String) {
        self.position_in_memory = String::from(p);
    }
    //重设
    pub fn reset(&mut self) {
        self.registers = [0 as i64; 64];
        self.position_in_memory = String::new();
        self.memory = [0 as u128; 0x1000];
    }
    //读取指令，分类
    fn type_opcode(code: &str) -> Type {
        match code {
            "0110011" => Type::R,
            "0010011" => Type::I,
            "0000011" => Type::I_LI,
            "0100011" => Type::S,
            "1100011" => Type::B,
            "0110111" => Type::U_LUI,
            "0010111" => Type::U_AUIPC,
            "1101111" => Type::J,
            _ => Type::Error,
        }
    }
    fn binary_run_R(code: String) {
        let mut run = R::default();
        run.binary_auto_run(code);
    }
}

//定义R类型指令
//
// opcode -> 操作码   占了7bit，在指令格式的0-6bit位上
// R类型的操作码为0110011
// rd -> 目标寄存器     占了5bit，在指令格式的7-11bit位上
// funct3 funct7 -> 两个操作字段     funct3占了3bit，在指令格式的12-14bit位上；funct7占了7bit，在指令格式的20-26bit位上。
// 操作字符add sub sll slt sltu xor srl sra or and
// rs1 -> 第一个源操作数寄存器    占了5bit，在指令格式的15-19bit位上。
// rs2 -> 第二个源操作数寄存器    占了5bit，在指令格式的27-31bit位上。
#[derive(Default)]
struct R {
    code: String,
    rd: usize,
    funct3: usize,
    funct7: usize,
    rs1: usize,
    rs2: usize,
}

impl R {
    //输入指令
    fn input_code(&mut self, code: String) {
        self.code = String::from(code);
    }
    //分离信息
    fn get_info(&mut self) {
        self.rd = String::from(&self.code[21..25]).parse().unwrap();
        self.funct3 = String::from(&self.code[18..20]).parse().unwrap();
        self.rs1 = String::from(&self.code[13..17]).parse().unwrap();
        self.funct7 = String::from(&self.code[6..12]).parse().unwrap();
        self.rs2 = String::from(&self.code[0..5]).parse().unwrap();
    }
    //运行函数
    pub fn binary_auto_run(&mut self, code: String) {
        self.input_code(code);
        self.get_info();
    }
    pub fn asm_auto_run() {}
}

pub struct ASM{
    riscv: RV,
}

impl Default for ASM{
    fn default() -> Self {
        Self{
            riscv : RV::default(),
        }
    }
}

impl ASM {
    /*
    以下是对汇编的实现
     */
    fn mov(&mut self, rd: usize, value: usize) {
        self.riscv.registers[rd] = value as i64;
    }
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] + self.riscv.registers[rs2];
    }
    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] - self.riscv.registers[rs2];
    }
    fn mul(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] * self.riscv.registers[rs2];
    }
    fn div(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = (self.riscv.registers[rs1] / self.riscv.registers[rs2]) as i64;
    }
    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] << self.riscv.registers[rs2];
    }
    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] >> self.riscv.registers[rs2];
    }
    fn xor(&mut self, rd: usize, rs1: usize ,rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] ^ self.riscv.registers[rs2];
    }
    fn or(&mut self, rd: usize, rs1: usize ,rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] | self.riscv.registers[rs2];
    }
    fn and(&mut self, rd: usize, rs1: usize ,rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] & self.riscv.registers[rs2];
    }
}
