/*
作者:Shirasawa-CN(Cpointerz)
贡献者:
创建时间:2022/8/24
最后维护时间:2022/8/25
 */

use std::any::Any;

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
    WSSB, //我是傻逼
}

//定义RISC-V的数据结构
struct RV {
    registers: [u128; 128],    //定义寄存器
    position_in_memory: usize, //定义记录内存位置
    memory: [u128; 0x1000],
}

impl Default for RV {
    fn default() -> Self {
        Self {
            registers: [0; 128],
            position_in_memory: 0,
            memory: [0; 0x1000],
        }
    }
}

impl RV {
    //重设
    fn reset(&mut self) {
        self.registers = [0 as u128; 128];
        self.position_in_memory = 0 as usize;
        self.memory = [0 as u128; 0x1000];
    }
    //读取指令，分类
    fn type_opcode(code: usize) -> Type {
        match code {
            0110011 => Type::R,
            0010011 => Type::I,
            0000011 => Type::I_LI,
            0100011 => Type::S,
            1100011 => Type::B,
            0110111 => Type::U_LUI,
            0010111 => Type::U_AUIPC,
            1101111 => Type::J,
            _ => Type::Error,
        }
    }
    //提取出opcode
    fn read_opcode(&self) -> usize {
        let p = self.position_in_memory;
        let result = ((p & 00000000000000000000000001111111) as usize);
        result
    }
    //读取指令
    fn read_position_in_memory(&mut self, p: usize) {
        self.position_in_memory = p;
    }
    //运行
    fn run(&mut self, value: usize) -> Type{
        self.read_position_in_memory(value);
        let opcode = self.read_opcode();
        let riscv_type = Self::type_opcode(opcode);
        riscv_type
    }
}

//定义R类型指令
//
// opcode -> 操作码   占了7bit，在指令格式的0-6bit位上
// R类型的操作码为0110011
// rd -> 目标寄存器     占了5bit，在指令格式的7-11bit位上
// funct3 funct7 -> 两个操作字段     funct3占了3bit，在指令格式的12-14bit位上；funct7占了7bit，在指令格式的25-31bit位上。
// 操作字符add sub sll slt sltu xor srl sra or and
// rs1 -> 第一个源操作数寄存器    占了5bit，在指令格式的15-19bit位上。
// rs2 -> 第二个源操作数寄存器    占了5bit，在指令格式的25-31bit位上。
struct R {
    opcode: u8,
    rd: u8,
    funct3: u8,
    funct7: u8,
    rs1: u8,
    rs2: u8,
}

impl R {
    fn new(opcode: u8, rd: u8, funct3: u8, funct7: u8, rs1: u8, rs2: u8) -> Self {
        Self {
            opcode,
            rd,
            funct3,
            funct7,
            rs1,
            rs2,
        }
    }
}
