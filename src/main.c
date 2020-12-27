/******************************************************************************
*
* Copyright (C) 2009 - 2014 Xilinx, Inc.  All rights reserved.
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* Use of the Software is limited solely to applications:
* (a) running on a Xilinx device, or
* (b) that interact with a Xilinx device through a bus or interconnect.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
* XILINX  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
* WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
* OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
* Except as contained in this notice, the name of the Xilinx shall not be used
* in advertising or otherwise to promote the sale, use or other dealings in
* this Software without prior written authorization from Xilinx.
*
******************************************************************************/

/*
 *
 *
 * This application configures UART 16550 to baud rate 9600.
 * PS7 UART (Zynq) is not initialized by this application, since
 * bootrom/bsp configures it to baud rate 115200
 *
 * ------------------------------------------------
 * | UART TYPE   BAUD RATE                        |
 * ------------------------------------------------
 *   uartns550   9600
 *   uartlite    Configurable only in HW design
 *   ps7_uart    115200 (configured by bootrom/bsp)
 */

// Main program for exercise

//****************************************************
//By default, every outputs used in this exercise is 0
//****************************************************
#include <stdio.h>
#include <stdbool.h>
#include "platform.h"
#include "xil_printf.h"
#include "sleep.h"
#include "xgpiops.h"
#include "xttcps.h"
#include "xscugic.h"
#include "xparameters.h"
#include "Pixel.h"
#include "Interrupt_setup.h"

//***Hint: Use sleep(x)  or usleep(x) if you want some delays.****
//to call assember code found in blinker.S, call it using "blinker();".


//Comment this if you want to disable all interrupts
#define enable_interrupts
//check game status
const uint8_t MAX=8;
bool game_continue=true;
uint8_t points[8];
uint8_t bullet_used=0;

//shooter
uint8_t shooter_x=1;
int8_t shooter_dir=0;
bool m_s_pressed=false;
//alien
uint8_t alien_x=0;
int8_t alien_dir=0;
int8_t alien_m_dir=0;
int8_t a_going_dir=1;
uint8_t const alien_y=0;


//bullet
bool bullet_build_b=false;
uint8_t bullet_x=1;
uint8_t bullet_y=5;
bool shoot_done=true;//assume was shooted




uint32_t* led=0x41200000;
int main()
{
	//**DO NOT REMOVE THIS****
	    init_platform();
	//************************


#ifdef	enable_interrupts
	    init_interrupts();
#endif

	    //setup screen
	    setup();


	    Xil_ExceptionEnable();


		//Main loop
		while(1){

		}


		cleanup_platform();
		return 0;
}



//Interrupt handler for switches and buttons. Connected buttons and switches are at bank2. Reading Status will tell which button or switch was used
void ButtonHandler(void *CallBackRef, u32 Bank, u32 Status)
{
	//****Write code here ****
	if(Status & 0b1){
		 shooter_dir=1;// led|=0x01;
	}
	else if(Status & 0b10){
		 shooter_dir=-1;//led|=1;
	}

	else if((Status & 0b100)&&shoot_done){//button 2 pressed to shoot

		bullet_build_b=true;
		bullet_x=shooter_x;// bullet will be shooted from same x as shooter has
		bullet_y=5;
		shoot_done=false;
		bullet_used++;
	}






	//****END OF OWN CODE*****************
}

//Timer interrupt handler for led matrix update. Frequency is 800Hz
void TickHandler(void *CallBackRef){
	//Don't remove this
	uint32_t StatusEvent;

	//exceptions must be disabled when updating screen
	Xil_ExceptionDisable();

	//****Write code here ****
	if(game_continue){
		build_shooter(&shooter_x, shooter_dir);//move_shooter
		shooter_dir=0;//move_shooter

		build_alien(&alien_x,alien_dir);//move_alien
		alien_dir=0;//move_alien

		//bullet
		if(bullet_build_b){
			SetPixel(bullet_x,bullet_y,0,255,0);
			if(bullet_y==0){//bullet rajalla
				SetPixel(bullet_x,bullet_y,0,0,0);
				shoot_done=true;

				if(bullet_x==alien_x){//writting game_point
					points[bullet_used-1]=1;
				}
				else if(bullet_x!=alien_x){//writting game_point
					points[bullet_used-1]=0;
				}
			}
		}

	}

	else{//game ends
		clear_pattern();
		for(int i=0;i<8;i++){
			if(points[i]==0){
				SetPixel(i,7,0,0,255);
			}
			else{
				SetPixel(i,7,0,255,0);
			}
		}
	}










	//****END OF OWN CODE*****************

	//*********clear timer interrupt status. DO NOT REMOVE********
	StatusEvent = XTtcPs_GetInterruptStatus((XTtcPs *)CallBackRef);
	XTtcPs_ClearInterruptStatus((XTtcPs *)CallBackRef, StatusEvent);
	//*************************************************************
	//enable exeptions
	Xil_ExceptionEnable();
}


//Timer interrupt for moving alien, shooting... Frequency is 10Hz by default
void TickHandler1(void *CallBackRef){

	//Don't remove this
	uint32_t StatusEvent;

	//****Write code here ****
	if(game_continue){
		give_alien_dir(alien_x, &alien_dir, &a_going_dir);

		//bullet
		if(bullet_y!=0){
			SetPixel(bullet_x,bullet_y,0,0,0);
			bullet_y--;
		}
		else{//jos on rajalla
			bullet_build_b=false;//will now build bullet anymore
			//check_game_status
			if(bullet_used==MAX){
				game_continue=false;
			}
		}


	}


	//****END OF OWN CODE*****************
	//clear timer interrupt status. DO NOT REMOVE
	StatusEvent = XTtcPs_GetInterruptStatus((XTtcPs *)CallBackRef);
	XTtcPs_ClearInterruptStatus((XTtcPs *)CallBackRef, StatusEvent);

}
