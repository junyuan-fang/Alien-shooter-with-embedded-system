use crate::xil;
use crate::LED_ADDRESS;

const PAGE_SIZE: usize = 10;

pub static mut PAGE: usize = 0;

/// Table for dots. Indices are page, x, y, color. Initialized to zero.

pub const channel:*mut u8 = 0x41220000 as *mut u8;
static mut DOTS: [[[u8;3 ]; 8]; 8] = [[[0; 3]; 8]; 8];

pub const ctrl:*mut u8 = 0x41220008 as *mut u8;//uint8_t *ctrl=0x41220008;//signal control

pub unsafe fn setup_led_matrix() {
    // Tip: use the following to set an ADDRESS to zero:
    /*
    core::ptr::write_volatile(ADDRESS, 0);
    */

    // The screen must be reset at start
    // Tip: use the following one-liners to flip bits on or off at ADDRESS. Oh
    // yes, it's a zero-cost lambda function in an embedded application.
    /*
    mutate_ptr(ADDR, |x| x | 1);
    mutate_ptr(ADDR, |x| x ^ 1);
    */
//c:
	
	mutate_ptr(ctrl, |x| x | 0x1);//*ctrl|=1;//reset

	//Write code that sets 6-bit values in register of DM163 chip.
	//Recommended that every bit in that register is set to 1. 6-bits and 24 "bytes",
	//so some kind of loop structure could be nice

	mutate_ptr(ctrl, |x| x | 0x10);//*ctrl|=0x10;//data's value is 1

	mutate_ptr(ctrl, |x| x ^ 0x04);//*ctrl &= !4; 					// Set SB to L for storing to 6-bit register bank.
		for  i in 0..24 {

			for j in 0..8 {

				// Data for 6-bit registers.
				let  mut data:  u8= 0b111111;

				for k in 0..6 {
				//while ( data ) {

					// If msb of data is 1 set channel open.
					if  data & 0x20 !=0   {
						mutate_ptr(ctrl, |x| x | 0x10);//*ctrl |= 0x10; 			//SET only BIT4 to 1 in control signal (SDA bit)
					} else {
						mutate_ptr(ctrl, |x| x ^ 0x10);//*ctrl &= !0x10;			//SET only BIT4 to 0 in control signal (SDA bit)
					}

					mutate_ptr(ctrl, |x| x ^ 0x08);//*ctrl &= !0x08; 				//SET only BIT3 to 0 in control signal (CLK bit)
					data <<= 1; 				//shift one to left
					mutate_ptr(ctrl, |x| x | 0x08);//*ctrl |= 0x08; 				//SET only BIT3 to 1 in control signal (CLK bit)
				}
			}
		}
	latch();								//release the data from the buffer
	mutate_ptr(ctrl, |x| x | 0x04);//*ctrl|=0x04;						//SB to 1, all following data will going to RGB buffer, won't go to brightness buffer anymore


    // TODO: Write code that sets 6-bit values in register of DM163 chip. It is
    // recommended that every bit in that register is set to 1. 6-bits and 24
    // "bytes", so some kind of loop structure could be nice
}

/// Set the value of one pixel at the LED matrix. Function is unsafe because it
/// uses global memory
pub unsafe fn set_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    // TODO: Set new pixel value. Take the parameeters and put them into the
    // DOTS array.
    DOTS[x][y][0]=r;
	DOTS[x][y][1]=g;
	DOTS[x][y][2]=b;
	run(x);
}

/// Refresh new data into the LED matrix. Hint: This function is supposed to
/// send 24-bytes and parameter x is for x-coordinate.
pub unsafe fn run(x: usize) {
    // TODO: Write into the LED matrix driver (8-bit data). Use values from DOTS
    // array.
    let mut data:u8=0;
	for y in 0..8 {
		for color in 0..3{
				data =DOTS[x][y][color]; //Binary value
			for byte_count in 0..8{
				//uint8_t data=0b11010110; //Binary value
				if data & 0x80 != 0 { *ctrl|=0x10;}  		//SET only BIT4 to 1 in control signal (SDA bit)
				else {*ctrl&=!0x10;} 					//SET only BIT4 to 0 in control signal (SDA bit)
				mutate_ptr(ctrl, |x| x ^ 0x08);//*ctrl&=!0x08; 						//SET only BIT3 to 0 in control signal (CLK bit)
				data<<=1; 							//shift one to left
				mutate_ptr(ctrl, |x| x | 0x08);//*ctrl|=0x08; 						//SET only BIT3 to 1 in control signal (CLK bit)
			}
		}
	}
	latch();
	open_line(x as u8);


}

/// Latch signal for the colors shield. See colorsshield.pdf for how latching
/// works.
unsafe fn latch() {
	mutate_ptr(ctrl, |x| x | 0x02);//*ctrl |= 0x02;//put on, data read from the buffer
	mutate_ptr(ctrl, |x| x ^ 0x02);//*ctrl &= !0x02;
    // TODO: Do the latching operation
}

/// Sets one line, matching with the parameter, as active.
pub unsafe fn open_line(i: u8) {
    // TODO: Write code here.
    // Tip: use a `match` statement for the parameter:

	 core::ptr::write_volatile(channel, 1<<i);
    /*
    match i {
        0 => {},
        _ => {},
    }
    */
}

/// A helper one-liner for mutating raw pointers at given address. You shouldn't need to change this.
pub unsafe fn mutate_ptr<A, F>(addr: *mut A, mutate_fn: F)
where
    F: FnOnce(A) -> A,
{
    let prev = core::ptr::read_volatile(addr);
    let new = mutate_fn(prev);
    core::ptr::write_volatile(addr, new);
}


///Game logic
pub unsafe fn clear_shooter(location: usize){
	let y: usize=7;
	set_pixel(location-1,y,0,0,0);
	set_pixel(location,y,0,0,0);
	set_pixel(location,y-1,0,0,0);
	set_pixel(location+1,y,0,0,0);
}

pub unsafe fn build_shooter( location: &mut usize ,  dir: i8){
	let y: usize=7;
	clear_shooter(*location);

	*location=(*location as i8 + dir as i8) as usize;//*location+=dir;
	if(*location>6){*location=6;}
	else if(*location<1){ *location=1;}


	set_pixel(*location-1,y,0,0,255);
	set_pixel(*location,y,0,0,255);
	set_pixel(*location,y-1,0,0,255);
	set_pixel(*location+1,y,0,0,255);
}


pub unsafe fn give_alien_dir( x: usize, dir: &mut i8,  a_going_dir: &mut i8){
	if(x==0) {
		*dir=1;
		*a_going_dir=1;
	}
	else if(x==7) {
		*dir=-1;
		*a_going_dir=-1;
	}
	else{
		*dir=*a_going_dir;
	}
}

pub unsafe fn build_alien(x:&mut usize, dir:  i8){
	set_pixel(*x,0,0,0,0);

	*x=(*x as i8+ dir as i8) as usize;//*x+=dir;
	set_pixel(*x,0,255,0,0);
}

pub unsafe fn clear_pattern(){
	for x in 0..7{
		for y in 0..7{
			set_pixel(x,y,0,0,0);
		}
	}
}