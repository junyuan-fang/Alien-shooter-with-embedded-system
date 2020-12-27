//! Implements the led-blinker course work on Xilinx Pynq-Z1 SoC.

// The `no_std` attribute prevents Rust standard library from being built into the binary. This is
// necessary, because the standard library is not available for baremetal Cortex-A9. The Rust `std`
// is in many ways similar to the C++ `std`.
#![no_std]
// Open feature gates to some particular extras related to low-level hacking that are not part of
// the core Rust language: assembler, language keyword overwriting and program entry point.
#![feature(llvm_asm,asm, lang_items, start)]

mod interrupt;
mod pixel;
mod print;

// xil_sys contains the Xilinx Cortex-A9 board support package (BSP) and a Rust
// FFI. We rename the module here as `xil`.
use xil_sys as xil;

// Re-import symbols from pixel without the `pixel::` prefix
use pixel::*;

// Rust `core` imports for using C-style void-pointers and info for a custom
// panic implementation.
use core::{ffi::c_void, panic::PanicInfo};

// Declare static globals like in the C-version. This is a reasonable way of
// communicating between threads in interrupt-driven concurrency.
pub static mut A_GLOBAL: usize = 0;

// Define the address of the ordinary LED interface in physical memory. Putting
// bits into the LED address the right way may cause desired blinking of
// hardware LEDs.
// FIXME: 0x00000000 is not the LED address. The correct address can be found in
// some of the provided documentation.
pub const LED_ADDRESS: *mut u8 = 0x00000000 as *mut u8;

const  MAX: usize=8;
static mut  game_continue: bool=true;
static mut points: [usize; 8]=[0;8];
static mut bullet_used: usize=0;

//shooter
static mut shooter_x: usize=1;
static mut shooter_dir: i8=0;
static mut  m_s_pressed: bool=true;
//alien

static mut alien_x: usize=0;
static mut alien_dir: i8=0;
static mut alien_m_dir: i8=0;
static mut a_going_dir: i8=1;
const  alien_y: usize=0;


//bullet
static mut  bullet_build_b: bool=false;
static mut bullet_x: usize=1;
static mut bullet_y: usize=5;
static mut  shoot_done: bool=true;//assume was shooted




static mut r_0: u32=0;
static mut r_1: u32=0;
static mut r_2: u32=0;
static mut r_3: u32=0;



// The #[start] attribute is usually not necessary, but we need to show the
// cross-compiler where to start executing. The underscore before the argument
// signals that the parameter is not used.
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialize board interrupt functions
    // N.B. Do not touch this function, concurrency is set up here
    interrupt::init();

    // An unsafe block for setting up the LED-matrix using the C-API, and for
    // touching a static global.

	let led=0x41200000 as *mut u32;
	unsafe {mutate_ptr(led, |x| x | 0b01);}

    unsafe {
        setup_led_matrix();
        // Setting a static global requires an `unsafe` block in Rust, because the
        // compiler cannot verify soundness in a case where an interrupt causes
        // simultaneous access from another thread. Thus we must make sure ourselves,
        // not to do that.
        A_GLOBAL = 0;
    }

    unsafe {
        // Enables the board to break execution of the main thread using an interrupt
        // request (IRQ), and jump onto the interrupt handler. Direct calls to C API
        // functions (`xil::*`) are `unsafe` by default, because the compiler
        // does not verify soundness of C code.
        xil::Xil_ExceptionEnable();
    }

    // Prints up to 64 characters using standard Rust [print formatting](https://doc.rust-lang.org/std/fmt/index.html).
    println64!("Hello Rust!");

    // Empty loop to keep the program running while the interrupt handlers do all the
    // work
    loop {}
}

/// Interrupt handler for switch and buttons. This function gets called on
/// switch and button interrupts. Connected buttons are at bank 2.
///
/// # Arguments
///
/// * `status` - a binding containing one flipped bit to match the source of the
///   interrupt. See line comments of contained `match` statement.
pub unsafe extern "C" fn button_handler(callback_ref: *mut c_void, _bank: u32, status: u32) {
    // Don't mind me, line is for brevity
    // N.B. Removing this line is totally okay
    let _gpio = callback_ref as *mut xil::XGpioPs;

	//set_pixel(5,5,0,255,0);

	match status{
		0b000001 => {shooter_dir=1;},
		0b000010 => {shooter_dir=-1;},
		0b000100 if shoot_done => {		
			bullet_build_b=true;
			bullet_x=shooter_x;// bullet will be shooted from same x as shooter has
			bullet_y=5;
			shoot_done=false;
			bullet_used+=1;
		},
		 _ => {},
	}

    // TODO: Write code here
    // Tip: use a match-statement to pattern-match button status. The match
    // statement takes the `status` parameter binding and matches it to
    // different binary patterns (eg. 0b001 for decimal 1, or 0b100 for
    // decimal 4). You can use binary, decimal or hex for the match, but I
    // found the binary representation more readable.
    /*
    match status {
        // No buttons are pressed
        0b000000 => {},
        // TODO: match into a pattern here
        // ??? => ???
        // `_` is the 'rest' pattern, that is handled if no other variant matches above
        _ => {},
    }
    */
    // End of your code
}

/// Timer interrupt handler for led matrix update. The function updates only one
/// line (`CHANNEL`) of the matrix per call, but sets `channel` as the next line
/// to be updated. `pub extern "C"` qualifier is required to allow passing the
/// handler to the C API.
pub unsafe extern "C" fn tick_handler(callback_ref: *mut c_void) {
    // Exceptions need to be disabled during screen update.
    xil::Xil_ExceptionDisable();
	
    // TODO: Write code here
	if game_continue {
		build_shooter(&mut shooter_x, shooter_dir);//move_shooter
		shooter_dir=0;//move_shooter

		build_alien(&mut alien_x,alien_dir);//move_alien
		alien_dir=0;//move_alien

		//bullet
		if bullet_build_b {
			set_pixel(bullet_x,bullet_y,0,255,0);
			if bullet_y==0 {//bullet rajalla
				set_pixel(bullet_x,bullet_y,0,0,0);
				shoot_done=true;

				if bullet_x==alien_x {//writting game_point
					points[bullet_used-1]=1;
				}
				else if bullet_x!=alien_x {//writting game_point
					points[bullet_used-1]=0;
				}
			}
		}

	}
	else{//game ends
		clear_pattern();
		for i in 0..8 {
			if(points[i]==0){
				set_pixel(i,7,0,0,255);
			}
			else{
				set_pixel(i,7,0,255,0);
			}
		}
	}





    // End of your code

    // Cast `void*` received from the C API to the "Triple Timer Counter" (TTC)
    // instance pointer. The C API needs to use void pointers to pass data around,
    // because the C specification does not describe abstract data types (ADT).
    let ttc = callback_ref as *mut xil::XTtcPs;

    // Clear timer interrupt status
    // N.B. Do not remove
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);

    // Put exceptions back on (previously disabled at the start of the interrupt
    // handler)
    xil::Xil_ExceptionEnable();
}

/// Timer interrupt handler for moving the alien, shooting, and other game
/// logic. See also [tick_handler](fn.tick_handler.html) and its line comments
/// for details.
pub unsafe extern "C" fn tick_handler_1(callback_ref: *mut c_void) {
    // TODO: Write code here

	unsafe {///// blinker
	llvm_asm!("
blinker:

	push {r0,r1,r2,r3}				
	LDR r0,=0x41200000 
	LDR r1,[r0]
	LDR r3,[r2]
	CMP r1,#0b01
	BEQ branch0
	B	branch1

branch0://shift to left
	MOV r3,#0b1// 0b1suunta on vasemmalle, 0b0 oikealle
	STR r3,[r2]
	B shift_left
branch1://shift to right or left
	CMP r1,#0b01000
	MOVEQ r3,#0b0
	STREQ r3,[r2]

	CMP r3,#0b1
	BEQ shift_left
	B	shift_right

shift_left:
	LSL r1,r1,#1
	STR r1,[r0]						
	pop {r3,r2,r1,r0}				
	bx lr						
shift_right:
	LSR r1,r1,#1
	STR r1,[r0]						
	pop {r3,r2,r1,r0}				
	bx lr"
	:
	:"r"(r_0),"r"(r_1), "r"(r_2), "r"(r_3)
	:
	:
	);
	}



	if game_continue{
	give_alien_dir(alien_x, &mut alien_dir as &mut i8 , &mut a_going_dir as &mut i8);

	//bullet
	if bullet_y!=0 {
		set_pixel(bullet_x,bullet_y,0,0,0);
		bullet_y-=1;
	}
	else{//jos on rajalla
		bullet_build_b=false;//will now build bullet anymore
		//check_game_status
		if bullet_used==MAX {
			game_continue=false;
		}
	}


	}







    // End of your code

    // Clear timer interrupt status
    // N.B. Do not remove
    let ttc = callback_ref as *mut xil::XTtcPs;
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);
}

/// A custom panic handler for Cortex-A9
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // logs "panicked at '$reason', src/main.rs:27:4" to host stdout
    println64!("{}", info);

    loop {}
}
