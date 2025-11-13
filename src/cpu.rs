pub struct CPU {
    pub register_a: u8, // Registrador A
    pub register_x: u8, // Registrador X
    pub register_y: u8, // Registrador Y
    pub status: u8,
    pub program_counter: u16, // Contador de programa
    pub stack_pointer: u8,    // Ponteiro da pilha
                              //memory: [u8; 0xFFFF], // Memória da CPU
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            stack_pointer: 0,
        }
    }

    pub fn interpret(&mut self, ROM: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode: u8 = ROM[self.program_counter as usize];
            self.program_counter += 1;

            //Verificar o que representa esse opcode em um switch case
            match opcode {
                //Caso tenha esse opcode, faça tal
                //LDA = Adiciona o prox byte
                0xA9 => {
                    let param = ROM[self.program_counter as usize];
                    self.program_counter += 1;

                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010; // Liga o Z
                    } else {
                        self.status = self.status & 0b1111_1101 // Desliga o Z
                    };

                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000 // Liga o N
                    } else {
                        self.status = self.status & 0b0111_1111 // Desliga o N
                    }
                }

                0x00 => {
                    return;
                }

                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
