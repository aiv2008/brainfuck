mod opcode;

use opcode::{Code, Opcode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 第一个参数就是传递的文件路径，例如：brainfuck res/hello_world.bf
    let data = std::fs::read(&args[1])?;
    // 转换为 Opcode 的数组
    let code = Code::from(data)?;
    println!("{:?}", code.instrs);
    Ok(())
    //unimplemented!()
}
