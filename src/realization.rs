/*
作者:Shirasawa-CN(Cpointerz)
贡献者:
创建时间:2022/8/24
最后维护时间:2022/8/26
 */

use tracing::{error, info, warn};

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
    //运行
    pub fn auto_run(&mut self, value: usize) {
        loop {
            self.read_position_in_memory(value);
            let opcode = self.read_opcode();
            let riscv_type = Self::type_opcode(opcode);

            match riscv_type {
                Type::R => self.run_R(),
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
    fn read_opcode(&self) -> usize {
        let p = self.position_in_memory;
        let result = ((p & 0x10F447) as usize);
        result
    }
    //读取指令
    fn read_position_in_memory(&mut self, p: usize) {
        self.position_in_memory = p;
    }
    //重设
    pub fn reset(&mut self) {
        self.registers = [0 as u128; 128];
        self.position_in_memory = 0 as usize;
        self.memory = [0 as u128; 0x1000];
    }

    //分离R指令
    #[warn(overflowing_literals)]
    fn run_R(&self) {
        //分离信息
        let p = self.position_in_memory;

        let rd = ((p & 0x102B3511D80) as usize);
        let funct3 = ((p & 0x64F43391F000) as usize);
        let rs1 = ((p & 0x9A3233A1A35D8000) as usize);
        let rs2 = ((p & 0x178756E190B11BBC0000) as usize);
        let funct7 = ((p & 0xE06317A43A087635BA7000000) as usize);

        let run = R::new(rd, funct3, funct7, rs1, rs2);
        run.auto_run();
    }
    //读取指令，分类
    fn type_opcode(code: usize) -> Type {
        match code {
            0x1ADBB => Type::R,
            0x271B => Type::I,
            0xB => Type::I_LI,
            0x186AB => Type::S,
            0x10C8EB => Type::B,
            0x1AE1F => Type::U_LUI,
            0x277F => Type::U_AUIPC,
            0x10CD37 => Type::J,
            _ => Type::Error,
        }
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
    rd: usize,
    funct3: usize,
    funct7: usize,
    rs1: usize,
    rs2: usize,
}

impl R {
    pub fn new(rd: usize, funct3: usize, funct7: usize, rs1: usize, rs2: usize) -> Self {
        Self {
            rd,
            funct3,
            funct7,
            rs1,
            rs2,
        }
    }

    pub fn auto_run(&self) {
        todo!("我是傻逼");
    }
}
