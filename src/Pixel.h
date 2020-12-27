/*
 * Pixel.h
 *
 *  Created on: ------
 *      Author: ------
 */

#ifndef PIXEL_H_
#define PIXEL_H_

#include <stdbool.h>
#include "platform.h"
#include "xil_printf.h"
#include "sleep.h"
#include "xgpiops.h"
#include "xttcps.h"
#include "xscugic.h"
#include "xparameters.h"

//size can be changed if needed
#define Page_size 10


void setup();
void SetPixel(uint8_t x,uint8_t y, uint8_t r, uint8_t g, uint8_t b);
void run(uint8_t c);
void latch();
void open_line(uint8_t i);
//game logic
void clear_shooter(uint8_t location);
void build_shooter(uint8_t* location, int8_t dir);

void give_alien_dir(uint8_t x, int8_t* dir, int8_t* a_going_dir);
void build_alien(uint8_t*x, int8_t dir);

//move_bullet
void clear_bullet(uint8_t x,uint8_t y);
void build_bullet(uint8_t x,uint8_t y);
void move_bullet(uint8_t* y);

void clear_pattern();


#endif /* PIXEL_H_ */
