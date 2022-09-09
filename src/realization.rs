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
    //定义记录内存位置
    memory: [u128; 0x1000],
}

impl Default for RV {
    fn default() -> Self {
        Self {
            registers: [0; 64],
            memory: [0; 0x1000],
        }
    }
}

impl RV {
    //运行
    pub fn binary_auto_run(&mut self, value: String) {
        loop {}
    }
    //重设
    pub fn reset(&mut self) {
        self.registers = [0 as i64; 64];
        self.memory = [0 as u128; 0x1000];
    }
}

pub struct ASM {
    riscv: RV,
}

impl Default for ASM {
    fn default() -> Self {
        Self {
            riscv: RV::default(),
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
    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] ^ self.riscv.registers[rs2];
    }
    fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] | self.riscv.registers[rs2];
    }
    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        self.riscv.registers[rd] = self.riscv.registers[rs1] & self.riscv.registers[rs2];
    }
}
