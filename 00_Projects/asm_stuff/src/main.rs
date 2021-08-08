#![feature(asm)]

fn main() {
    
    unsafe {
        asm!("nop");
        asm!("mov eax, 1");
    }

}
