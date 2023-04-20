use std::collections::HashMap;

//定义一个枚举类型 Opcode 来代表以上的八种运算符，用ASCII码表示，然后编写一个转换函数将字节转换为 Opcode。由于 [ 与 ] 总是成双成对的出现且互相关联，代码内使用了 jtable 来存储它们之间的位置关系，以便快速决定跳转的目的地址。当然这不是必须的，也可以在解释 [ 和 ] 的时候实时的前向搜索或后向搜索以找到对应的符号位置。
#[derive(Debug)]
pub enum Opcode {
    SHR = 0x3e,
    SHL = 0x3c,
    ADD = 0x2b,
    SUB = 0x2d,
    PUTCHAR = 0x2e,
    GETCHAR = 0x2c,
    LB = 0x5b,
    RB = 0x5d,
}

impl  From<u8> for Opcode {
    fn from( u: u8)->Self{
        match u {
            0x3e => Opcode::SHR,
            0x3c => Opcode::SHL,
            0x2b => Opcode::ADD,
            0x2d => Opcode::SUB,
            0x2e => Opcode::PUTCHAR,
            0x2c => Opcode::GETCHAR ,
            0x5b => Opcode::LB,
            0x5d => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code{
    //指令数组
    pub instrs: Vec<Opcode>,
    //存储[和]位置关系
    pub jtable: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>{
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];
        let instrs: Vec<Opcode> = data.iter().filter(|x| dict.contains(x)).map(|x| Opcode::from(*x)).collect();

        // 借助栈结构来匹配 [ 和 ] 符号，然后存入 jtable 中
        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize,usize> = HashMap::new();
        for (i, e) in instrs.iter().enumerate(){
            //match *e {
            //    Opcode::LB => {jstack.push(i);}
            //    Opcode::RB => {
            //        jstack.push(i);
            //        let j = jstack.pop().ok_or("pop from empty stack")?;
            //        jtable.insert(j, i);
            //        jtable.insert(i, j);
            //    }
            //    _ => { continue;}
            //}
           if let Opcode::LB = *e{
                jstack.push(i);
           }
           if let Opcode::RB = *e{
                let j = jstack.pop().ok_or("pop from empty stack")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
           }
        }
        Ok(Code{instrs, jtable})
        //unimplemented!() 
    }
    
}
