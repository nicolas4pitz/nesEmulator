pub struct CPU {
    pub register_a: u8, // Registrador A
    pub register_x: u8, // Registrador X
    pub register_y: u8, // Registrador Y
    pub status: u8,
    pub memory: [u8; 0xFFFF], //memory: [u8; 0xFFFF], // Memória da CPU
    pub program_counter: u16, // Contador de programa
    pub stack_pointer: u8,    // Ponteiro da pilha
                              
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            memory: [0; 0xFFFF],
            program_counter: 0,
            stack_pointer: 0,
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
      self.memory[addr as usize]
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
      let lo = self.mem_read(pos) as u16;
      let hi = self.mem_read(pos + 1) as u16;
      (hi << 8) | (lo as u16)
    }

    fn mem_write(&mut self, addr: u16, data: u8){
      self.memory[addr as usize] = data;
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
      let hi = (data >> 8) as u8;
      let lo = (data & 0xff) as u8;
      self.mem_write(pos, lo);
      self.mem_write(pos + 1, hi);
    }

    // Reset vai restaurar o estado de todos os registradores, e inicializar o pc (program_counter) pelo segundo byte armazenado em 0xFFFC
    pub fn reset(&mut self){
      self.register_a = 0;
      self.register_x = 0;
      self.status = 0;

      self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset(); // No meio mesmo?
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>){
      self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
      self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self){
      loop {
            let opcode: u8 = self.mem_read(self.program_counter);
            self.program_counter += 1;

            //Verificar o que representa esse opcode em um switch case
            match opcode {

                // TAX = Carrega o acumulador A em X
                0xAA => self.tax(),
                    

                //Caso tenha esse opcode, faça tal
                //LDA = Adiciona o prox byte
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }

                0xE8 => self.inx(),

                0x00 => {
                    return;
                }

                _ => todo!(),
            }
        }
    }

    pub fn check_register_z_and_n(&mut self, register: u8){
        if register == 0 {
            self.status = self.status | 0b0000_0010; // Liga o Z
        } else {
            self.status = self.status & 0b1111_1101 // Desliga o Z
        };

        if register & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000 // Liga o N
        } else {
            self.status = self.status & 0b0111_1111 // Desliga o N
        }
    }

    fn lda(&mut self, param: u8){
        self.register_a = param;
        self.check_register_z_and_n(self.register_a);
    }

    fn tax(&mut self){
        self.register_x = self.register_a;
        self.check_register_z_and_n(self.register_x);
    }

    fn inx(&mut self){
      self.register_x = self.register_x.wrapping_add(1);
      self.check_register_z_and_n(self.register_x);
    }

    pub fn interpret(&mut self, ROM: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode: u8 = ROM[self.program_counter as usize];
            self.program_counter += 1;

            //Verificar o que representa esse opcode em um switch case
            match opcode {

                // TAX = Carrega o acumulador A em X
                0xAA => self.tax(),

                //Caso tenha esse opcode, faça tal
                //LDA = Adiciona o prox byte
                0xA9 => {
                    let param = ROM[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }

                0xE8 => self.inx(),

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
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
      let mut cpu = CPU::new();
      cpu.load(vec![0xaa, 0x00]);
      cpu.reset();
      cpu.register_a = 10;
      cpu.run();

      assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

      #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
  
        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }
}
