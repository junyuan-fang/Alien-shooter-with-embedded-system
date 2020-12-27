/*
 * Pixel.c
 *
 *  Created on: -----
 *      Author: -----
 */

#include "Pixel.h"
#include <stdbool.h>

/******use these if you know how*********
//Table for pixel dots. dots[page][X][Y][COLOR]
//volatile uint8_t dots[Page_size][8][8][3]={0};
//volatile uint8_t page=0;
****************************************/

//Table for pixel dots.
//				 dots[X][Y][COLOR]
volatile uint8_t dots[8][8][3]={0};

#define channel *(uint8_t*) 0x41220000

uint8_t *ctrl=0x41220008;//signal control


void setup(){

	//reseting screen at start is a MUST to operation. Hint 1: reset is active low.
	//Hint 2: Reset pin from CPU to DM163 is default 0
	*ctrl|=1;//reset

	//Write code that sets 6-bit values in register of DM163 chip.
	//Recommended that every bit in that register is set to 1. 6-bits and 24 "bytes",
	//so some kind of loop structure could be nice

	*ctrl|=0x10;//data's value is 1

	*ctrl &= ~4; 					// Set SB to L for storing to 6-bit register bank.
		for ( uint8_t i = 0; i < 24; i++ ) {

			for ( uint8_t j = 0; j < 8; j++ ) {

				// Data for 6-bit registers.
				uint8_t data = 0b111111;

				for ( uint8_t k = 0; k < 6; k++ ) {
				//while ( data ) {

					// If msb of data is 1 set channel open.
					if( data & 0x20 ) {
						*ctrl |= 0x10; 			//SET only BIT4 to 1 in control signal (SDA bit)
					} else {
						*ctrl &= ~0x10;			//SET only BIT4 to 0 in control signal (SDA bit)
					}

					*ctrl &= ~0x08; 				//SET only BIT3 to 0 in control signal (CLK bit)
					data <<= 1; 				//shift one to left
					*ctrl |= 0x08; 				//SET only BIT3 to 1 in control signal (CLK bit)
				}
			}
		}
	latch();								//release the data from the buffer
	*ctrl|=0x04;						//SB to 1, all following data will going to RGB buffer, won't go to brightness buffer anymore
}




//Set value of one pixel at led matrix
void SetPixel(uint8_t x,uint8_t y, uint8_t r, uint8_t g, uint8_t b){

	//Set new pixel value. Put function paremeter values to dots array at correct places

	dots[x][y][0]=r;
	dots[x][y][1]=g;
	dots[x][y][2]=b;
	run(x);

}


//Put new data to led matrix. Hint: This function is supposed to send 24-bytes and parameter x is for x-coordinate.
void run(uint8_t x){
	uint8_t data=0;
	for(uint8_t y=0; y<8; y++){
		for(uint8_t color=0; color< 3; color++){
				data =dots[x][y][color]; //Binary value
			for(uint8_t byte_count=0; byte_count<8; byte_count++){
				//uint8_t data=0b11010110; //Binary value
				if(data & 0x80) *ctrl|=0x10;  		//SET only BIT4 to 1 in control signal (SDA bit)
				else *ctrl&=~0x10; 					//SET only BIT4 to 0 in control signal (SDA bit)
				*ctrl&=~0x08; 						//SET only BIT3 to 0 in control signal (CLK bit)
				data<<=1; 							//shift one to left
				*ctrl|=0x08; 						//SET only BIT3 to 1 in control signal (CLK bit)
			}
		}
	}
	latch();
	open_line(x);

	//Write code that writes data to led matrix driver (8-bit data). Use values from dots array


}

//latch signal. See colorshield.pdf in project folder how latching works
void latch(){
	*ctrl |= 0x02;//put on, data read from the buffer
	*ctrl &= ~0x02;
}


//Set one line as active per time.
void open_line(uint8_t x){
	channel =(1<<x);
}

///Game logic
void clear_shooter(uint8_t location){
	uint8_t y=7;
	SetPixel(location-1,y,0,0,0);
	SetPixel(location,y,0,0,0);
	SetPixel(location,y-1,0,0,0);
	SetPixel(location+1,y,0,0,0);
}

void build_shooter(uint8_t* location, int8_t dir){
	uint8_t y=7;
	clear_shooter(*location);

	*location+=dir;
	if(*location>6)*location=6;
	else if(*location<1) *location=1;


	SetPixel(*location-1,y,0,0,255);
	SetPixel(*location,y,0,0,255);
	SetPixel(*location,y-1,0,0,255);
	SetPixel(*location+1,y,0,0,255);
}


void give_alien_dir(uint8_t x,int8_t* dir, int8_t* a_going_dir){
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

void build_alien(uint8_t*x, int8_t dir){
	SetPixel(*x,0,0,0,0);
	*x+=dir;
	SetPixel(*x,0,255,0,0);
}

void clear_pattern(){
	for(int x=0;x<8;x++ ){
		for(int y=0;y<8;y++ ){
			SetPixel(x,y,0,0,0);
		}
	}
}















