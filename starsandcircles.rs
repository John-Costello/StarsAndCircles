#![allow(unused_parens)]

extern crate rand;
use rand::Rng;
use nannou::prelude::*;

#[macro_use]
extern crate arrayref;

const NUM_OF_DOT: usize = 100;
const NUM_OF_CIRCLE: usize = 5;
const NUM_OF_STAR: usize = 10;
const WIDTH:u32=800;
const HEIGHT:u32=800;
const TWO_PI:f32=6.283185307179586476925286766559;
const PI:f32=3.1415926535897932384626433832795;
const HALF_PI:f32=PI/2.0;
static mut COLOUR_ARRAY:[[usize;3];4096]=[[0;3];4096]; 

fn main() {
	unsafe{COLOUR_ARRAY=init_colour_array();}
    nannou::app(model).size(WIDTH, HEIGHT).update(update).run();
}

//====================================================================================

struct Model {
    dots: [Dot; NUM_OF_DOT],
	circles: [Circle; NUM_OF_CIRCLE],
	stars: [Star; NUM_OF_STAR],
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let _window = app.window(window_id).unwrap();  
    let mut model:Model = Model {
        dots: [Dot::new() ; NUM_OF_DOT],
        circles: [Circle::new() ; NUM_OF_CIRCLE],	
        stars: [Star::new() ; NUM_OF_STAR],		
    };
	for dot in &mut model.dots
	{
		*dot=Dot::new();
	}
    for circle in &mut model.circles
	{
		*circle=Circle::new();
	}	
	for star in &mut model.stars
	{
		*star=Star::new();
	}
	return model;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for dot in &mut model.dots
	{
	    dot.update_dot();	
    }
	for circle in &mut model.circles
	{
	    circle.update_circle();	
    }
	for star in &mut model.stars
	{
	    star.update_star();	
    }	
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
	draw.background().color(BLACK);
    
    for dot in &model.dots
	{
		// https://docs.rs/nannou/latest/nannou/draw/primitive/ellipse/struct.Ellipse.html
		draw.ellipse()
            .xy( Vec2::new(dot.x, dot.y) )          
			.radius(dot.r)           
			//.no_fill()
			.color(WHITE)
            .stroke_weight(1.0)
            .stroke(WHITE);	
    }
	
	for circle in &model.circles
	{
		draw.ellipse()
		.xy( Vec2::new(circle.x, circle.y) )
		.radius(circle.r)
		.no_fill()
		.stroke_weight(circle.sw)
		.stroke(rgb( (unsafe{COLOUR_ARRAY[circle.col][0]} as f32)/255.,(unsafe{COLOUR_ARRAY[circle.col][1]} as f32)/255.,(unsafe{COLOUR_ARRAY[circle.col][2]} as f32)/255.));
	}
	
	for star in &model.stars
	{
		if(star.n==5)
		{
		draw.path()
		    .stroke()
			.weight(star.sw)			
			.color( rgb( (unsafe{COLOUR_ARRAY[star.col][0]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][1]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][2]} as f32)/255.) )
            .caps_round()
            .join_round()
			.points_closed(array_ref!(star.points, 0, 10).clone());
		}
        else if(star.n==6)
		{
		draw.path()
		    .stroke()
			.weight(star.sw)			
			.color( rgb( (unsafe{COLOUR_ARRAY[star.col][0]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][1]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][2]} as f32)/255.) )
            .caps_round()
            .join_round()
			.points_closed(array_ref!(star.points, 0, 12).clone());
		}
		else if(star.n==7)
		{
		draw.path()
		    .stroke()
			.weight(star.sw)			
			.color( rgb( (unsafe{COLOUR_ARRAY[star.col][0]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][1]} as f32)/255.,(unsafe{COLOUR_ARRAY[star.col][2]} as f32)/255.) )
            .caps_round()
            .join_round()
			.points_closed(star.points);
		}
		
        		
	}
	draw.to_frame(app, &frame).unwrap();
}

fn raw_window_event(app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
} 
//==================================================================

#[derive(Copy, Clone)]
struct Dot {
	x0:f32,   // x0 start point of travel-paths.
	y0:f32,   // y0 start point of travel-paths.
	x:f32,    // x value point on travel-path.
	y:f32,    // x value point on travel-path.
    r:f32,    // Radius of dot.
	a:f32,    // Angle of travel-path from center.
	len:f32,  // Overall length of travel-path.
	dist:f32, // Distance value into travel-path, initial distance.
	f:f32,    // f is fraction of travel-path traveled to determine sw strokeWeight value.
	sp:f32,   // Speed value, number between 1 and 2, pixels travel per frame.
	t:f32,    // Time value, initial time value determined from initial distance divided by speed.
}

impl Dot {
	fn new()->Dot{
	   let x0:f32=0.0; 
	   let y0:f32=0.0; 
	   let x:f32=x0;
	   let y:f32=y0;
	   let a:f32=TWO_PI*rand::thread_rng().gen_range(0.0..1.0);
	   let len:f32=(((WIDTH as f32/2.0)+10.0).pow(2.0)+((HEIGHT as f32/2.0)+10.0).pow(2.0)).sqrt()*(1.0+rand::thread_rng().gen_range(0.0..1.0));
	   let dist:f32=len*rand::thread_rng().gen_range(0.0..1.0);
	   let f:f32=dist/len;
	   let r:f32=if f>0.9{9.0/4.0} 
		   else if f>0.8{8.0/4.0}
		   else if f>0.7{7.0/4.0}
		   else if f>0.6{6.0/4.0}
		   else if f>0.5{5.0/4.0}
		   else if f>0.4{4.0/4.0}
		   else if f>0.3{3.0/4.0}
		   else if f>0.2{2.0/4.0}
		   else if f>0.1{1.0/4.0}
		   else {0.0};
	   let sp:f32=1.0*(1.0+rand::thread_rng().gen_range(0.0..1.0));
	   let t:f32=dist/sp;
	   return Dot{   x0:x0,
	                 y0:y0,
					 x:x,
					 y:y,
					 r:r,
					 a:a,
					 len:len,
					 dist:dist,
					 f:f,
					 sp:sp,
					 t:t,
					};
	}
	
	fn update_dot(&mut self) {
		self.x=self.a.cos()*self.sp*self.t;
		self.y=self.a.sin()*self.sp*self.t;
		self.dist=((self.x-self.x0).pow(2.0)+(self.y-self.y0).pow(2.0)).sqrt();
		self.f=self.dist/self.len;
		self.r=if self.f>0.9{9.0/4.0} 
		   else if self.f>0.8{8.0/4.0}
		   else if self.f>0.7{7.0/4.0}
		   else if self.f>0.6{6.0/4.0}
		   else if self.f>0.5{5.0/4.0}
		   else if self.f>0.4{4.0/4.0}
		   else if self.f>0.3{3.0/4.0}
		   else if self.f>0.2{2.0/4.0}
		   else if self.f>0.1{1.0/4.0}
		   else {0.0};
		self.t=self.t+1.0;
		if(self.dist>self.len)
		{
			self.t=0.;
			self.x=self.x0;
			self.y=self.y0;
			self.r=0.0;
			self.a=TWO_PI*rand::thread_rng().gen_range(0.0..1.0);
			self.len=(((WIDTH as f32/2.0)+10.0).pow(2.0)+((HEIGHT as f32/2.0)+10.0).pow(2.0)).sqrt()*(1.0+rand::thread_rng().gen_range(0.0..1.0));
			self.sp=1.0*(1.+rand::thread_rng().gen_range(0.0..1.0));
		}
	}
}
//=======================================================================

#[derive(Copy, Clone)]
struct Circle {
	x:f32,   
	y:f32,   
	a:f32,
	r:f32,
	sp:f32,
	sw:f32,
	col:usize,
	direction_forward:bool,
}

impl Circle {
	fn new()->Circle{
		let x:f32=rand::thread_rng().gen_range(0.0..1.0)*(WIDTH as f32)-((WIDTH as f32)/2.0);
		let y:f32=rand::thread_rng().gen_range(0.0..1.0)*(HEIGHT as f32)-((HEIGHT as f32)/2.0);
		let a:f32=TWO_PI*rand::thread_rng().gen_range(0.0..1.0);
		let r:f32=30.0+rand::thread_rng().gen_range(0.0..30.0);
		let sp:f32=1.0*(1.0+rand::thread_rng().gen_range(0.0..1.0));
		let sw:f32=1.5+rand::thread_rng().gen_range(0.0..3.5);
		let col:usize=rand::thread_rng().gen_range(0..4096);
		let direction_forward:bool= if rand::thread_rng().gen_range(0..4096)%2==0 {true} else {false};
	    return Circle{x:x, y:y, a:a, r:r, sp:sp, sw:sw, col:col, direction_forward:direction_forward,};
	}
	
	fn update_circle(&mut self) {
		self.x = self.x+self.a.cos()*self.sp;
		self.y = self.y+self.a.sin()*self.sp;
		if(self.x<((WIDTH as f32)/-2.0)+self.r){
			let mut a_new:f32=((((self.a%TWO_PI)+(HALF_PI))*(-1.0))-(HALF_PI))%TWO_PI;
		    while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new<=HALF_PI||a_new>=3.0*HALF_PI){   self.a=a_new;   }
	    }		
		else if(self.x>((WIDTH as f32)/2.0)-self.r){
			let mut a_new:f32=((((self.a%TWO_PI)+(HALF_PI))*(-1.0))-(HALF_PI))%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=HALF_PI && a_new<=3.0*HALF_PI){   self.a=a_new;   }
		}
        if(self.y<((HEIGHT as f32)/-2.0)+self.r){
			let mut a_new:f32=(-self.a)%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=0.0 && a_new<=PI){   self.a=a_new;   }
		}
        else if(self.y>((HEIGHT as f32)/2.0)-self.r){
			let mut a_new:f32=(-self.a)%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=PI && a_new<=TWO_PI){   self.a=a_new;   }
		}
		if(self.direction_forward==true)
		{
			self.col+=1;
			if(self.col==4095){   self.direction_forward=false;   }
		}
		else if(self.direction_forward==false)
		{
			self.col-=1;
			if(self.col==0){   self.direction_forward=true;   }
		}
	}
}
//=======================================================================
#[derive(Copy, Clone)]
struct Star{
	x:f32,   
	y:f32,   
	a:f32,
	r:f32,
	ra:f32,
	rb:f32,
	n:u8,
	rot: f32,
	rot_step: f32,
	sp:f32,
	sw:f32,
	col:usize,
	direction_forward:bool,
	points: [ Vec2; 14],
}

impl Star {
	fn new()->Star{
		let x:f32=rand::thread_rng().gen_range(0.0..1.0)*(WIDTH as f32)-((WIDTH as f32)/2.0);
		let y:f32=rand::thread_rng().gen_range(0.0..1.0)*(HEIGHT as f32)-((HEIGHT as f32)/2.0);
		let a:f32=TWO_PI*rand::thread_rng().gen_range(0.0..1.0);
		let r:f32=30.0+rand::thread_rng().gen_range(0.0..30.0);
		let ra:f32=r;
		let rb:f32=ra*(0.45+rand::thread_rng().gen_range(0.0..0.2));
		let n:u8=(5+(rand::thread_rng().gen_range(0..3)));
		let nf:f32= n as f32;
		let rot:f32=0.0;
		let rot_step:f32=((rand::thread_rng().gen_range(0.0..1.5))*TWO_PI/360.0)*(if rand::thread_rng().gen_range(0..2)%2==0{1.0} else {-1.0});
		let sp:f32=1.0*(1.0+rand::thread_rng().gen_range(0.0..1.0));
		let sw:f32=3.0+rand::thread_rng().gen_range(0.0..1.0);
		let col:usize=rand::thread_rng().gen_range(0..4096);
		let direction_forward:bool= if rand::thread_rng().gen_range(0..4096)%2==0 {true} else {false};
		let points:[ Vec2; 14]=[ pt2(x+ra*rot.cos(), y+ra*rot.sin()),
		                         pt2(x+rb*(rot+1.*PI/nf).cos(), y+rb*(rot+1.*PI/nf).sin()),
							     pt2(x+ra*(rot+2.*PI/nf).cos(), y+ra*(rot+2.*PI/nf).sin()),
		                         pt2(x+rb*(rot+3.*PI/nf).cos(), y+rb*(rot+3.*PI/nf).sin()),								 
                                 pt2(x+ra*(rot+4.*PI/nf).cos(), y+ra*(rot+4.*PI/nf).sin()),
		                         pt2(x+rb*(rot+5.*PI/nf).cos(), y+rb*(rot+5.*PI/nf).sin()),
							     pt2(x+ra*(rot+6.*PI/nf).cos(), y+ra*(rot+6.*PI/nf).sin()),
		                         pt2(x+rb*(rot+7.*PI/nf).cos(), y+rb*(rot+7.*PI/nf).sin()),
							     pt2(x+ra*(rot+8.*PI/nf).cos(), y+ra*(rot+8.*PI/nf).sin()),
		                         pt2(x+rb*(rot+9.*PI/nf).cos(), y+rb*(rot+9.*PI/nf).sin()),								 
                                 pt2(x+ra*(rot+10.*PI/nf).cos(), y+ra*(rot+10.*PI/nf).sin()),
		                         pt2(x+rb*(rot+11.*PI/nf).cos(), y+rb*(rot+11.*PI/nf).sin()),
							     pt2(x+ra*(rot+12.*PI/nf).cos(), y+ra*(rot+12.*PI/nf).sin()),
		                         pt2(x+rb*(rot+13.*PI/nf).cos(), y+rb*(rot+13.*PI/nf).sin())		 
								 ];
	    return Star{x:x, y:y, a:a, r:r, ra:ra, rb:rb, n:n, rot:rot, rot_step:rot_step,
		   sp:sp, sw:sw, col:col, direction_forward:direction_forward, points:points  };
	}
	
	fn update_star(&mut self) {
		self.x = self.x+self.a.cos()*self.sp;
		self.y = self.y+self.a.sin()*self.sp;
		if(self.x<((WIDTH as f32)/-2.0)+self.r){
			let mut a_new:f32=((((self.a%TWO_PI)+(HALF_PI))*(-1.0))-(HALF_PI))%TWO_PI;
		    while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new<=HALF_PI||a_new>=3.0*HALF_PI){   self.a=a_new;   }
	    }		
		else if(self.x>((WIDTH as f32)/2.0)-self.r){
			let mut a_new:f32=((((self.a%TWO_PI)+(HALF_PI))*(-1.0))-(HALF_PI))%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=HALF_PI && a_new<=3.0*HALF_PI){   self.a=a_new;   }
		}
        if(self.y<((HEIGHT as f32)/-2.0)+self.r){
			let mut a_new:f32=(-self.a)%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=0.0 && a_new<=PI){   self.a=a_new;   }
		}
        else if(self.y>((HEIGHT as f32)/2.0)-self.r){
			let mut a_new:f32=(-self.a)%TWO_PI;
			while(a_new<0.0){   a_new+=TWO_PI;   }
			if(a_new>=PI && a_new<=TWO_PI){   self.a=a_new;   }
		}
		if(self.direction_forward==true)
		{
			self.col+=1;
			if(self.col==4095){   self.direction_forward=false;   }
		}
		else if(self.direction_forward==false)
		{
			self.col-=1;
			if(self.col==0){   self.direction_forward=true;   }
		}
		
		self.rot = (self.rot+self.rot_step)%TWO_PI;
		let x=self.x;
		let y=self.y;
		let ra=self.ra;
		let rb=self.rb;
		let nf=self.n as f32;
		let rot=self.rot;
		
		self.points=[ pt2(x+ra*rot.cos(), y+ra*rot.sin()),
		                         pt2(x+rb*(rot+1.*PI/nf).cos(), y+rb*(rot+1.*PI/nf).sin()),
							     pt2(x+ra*(rot+2.*PI/nf).cos(), y+ra*(rot+2.*PI/nf).sin()),
		                         pt2(x+rb*(rot+3.*PI/nf).cos(), y+rb*(rot+3.*PI/nf).sin()),								 
                                 pt2(x+ra*(rot+4.*PI/nf).cos(), y+ra*(rot+4.*PI/nf).sin()),
		                         pt2(x+rb*(rot+5.*PI/nf).cos(), y+rb*(rot+5.*PI/nf).sin()),
							     pt2(x+ra*(rot+6.*PI/nf).cos(), y+ra*(rot+6.*PI/nf).sin()),
		                         pt2(x+rb*(rot+7.*PI/nf).cos(), y+rb*(rot+7.*PI/nf).sin()),
							     pt2(x+ra*(rot+8.*PI/nf).cos(), y+ra*(rot+8.*PI/nf).sin()),
		                         pt2(x+rb*(rot+9.*PI/nf).cos(), y+rb*(rot+9.*PI/nf).sin()),								 
                                 pt2(x+ra*(rot+10.*PI/nf).cos(), y+ra*(rot+10.*PI/nf).sin()),
		                         pt2(x+rb*(rot+11.*PI/nf).cos(), y+rb*(rot+11.*PI/nf).sin()),
							     pt2(x+ra*(rot+12.*PI/nf).cos(), y+ra*(rot+12.*PI/nf).sin()),
		                         pt2(x+rb*(rot+13.*PI/nf).cos(), y+rb*(rot+13.*PI/nf).sin())		 
								 ];
	}
}

//======================================================================
fn init_colour_array()->[[usize;3];4096]{
	let mut gen_colour_array:[[usize;3];4096]=[[0; 3]; 4096];
	let cube_length:usize=16;
	let cube_volume:usize=cube_length.pow(3);
	let mut shell_number:usize; //let mut ring_number:usize; let mut cell_number:usize;
	let mut modified_shell_sub_location_number:usize;
	let mut x_number:usize; let mut y_number:usize;	let mut z_number:usize;
	let mut r_number:usize; let mut g_number:usize; let mut b_number:usize;
	
	for location_index in (1..(cube_volume+1))
	{
		shell_number=shell_ring_cell_val(location_index)[0];
		//ring_number=shell_ring_cell_val(location_index)[1];
		//cell_number=shell_ring_cell_val(location_index)[2];
		modified_shell_sub_location_number=shell_ring_cell_val(location_index)[3];
		
		x_number=xyz_val(shell_number, modified_shell_sub_location_number)[0];
		y_number=xyz_val(shell_number, modified_shell_sub_location_number)[1];
		z_number=xyz_val(shell_number, modified_shell_sub_location_number)[2];

        r_number=(x_number-1)*(cube_length)+(x_number-1);
        g_number=(y_number-1)*(cube_length)+(y_number-1);
        b_number=(z_number-1)*(cube_length)+(z_number-1);
		
		gen_colour_array[location_index-1]=[r_number, g_number, b_number];		
	}
	return gen_colour_array;
}
//======================================================================
fn shell_ring_cell_val(location:usize)->[usize;4]{
	let shell_value:usize;
	let mut shell_index:usize;
	let ring_value:usize;
	let mut ring_index:usize;
	let cell_value:usize;
	let shell_sub_location:usize;
	let mut shell_sub_location_index:usize;
	let ring_sub_location:usize;
	let previous_ring_value:usize;
	let mut previous_ring_index:usize;
	let mut previous_rings_cells_index:usize;
	let previous_rings_cells_value:usize;
	let num_of_cells_per_shell_value:usize;
	let mut num_of_cells_per_shell_index:usize;
	let mut modified_shell_sub_location:usize=0;
	//---------------------
	shell_index=1;
	while location>shell_index.pow(3){
		shell_index+=1;
	}
	shell_value=shell_index;
	//----------------------
	shell_sub_location=location-(shell_value-1).pow(3);
	ring_index=1;
	shell_sub_location_index=1;
	while(shell_sub_location_index<shell_sub_location){
		ring_index+=1;
		shell_sub_location_index+=((6*ring_index)-6);
	}
	ring_value=ring_index;
	previous_ring_value=ring_value-1;
	previous_ring_index=1;
	previous_rings_cells_index=0;
	while(previous_ring_index<=previous_ring_value){
		if(previous_ring_index==1){
		    previous_rings_cells_index=1;
		}
		else{
			previous_rings_cells_index+=((6*previous_ring_index)-6);
		}
		previous_ring_index+=1;
	}
	previous_rings_cells_value=previous_rings_cells_index;
	ring_sub_location=shell_sub_location-previous_rings_cells_value;
	cell_value=ring_sub_location;
	//----------------------------------------------------
	num_of_cells_per_shell_index=1;
	ring_index=1;
	while(ring_index < shell_value){
		ring_index+=1;
		num_of_cells_per_shell_index+=((6*ring_index)-6);
	}
	num_of_cells_per_shell_value=num_of_cells_per_shell_index;
	//-----------------------------------------------------
	if(shell_value%2==0){
		modified_shell_sub_location=shell_sub_location;
	}
	else if(shell_value%2==1){
		modified_shell_sub_location=num_of_cells_per_shell_value+1-shell_sub_location;
	}
	//-------------------------------------------------------
	return([shell_value, ring_value, cell_value, modified_shell_sub_location]);
}
//======================================================================
fn xyz_val(shell_value:usize, modified_shell_sub_location: usize)->[usize;3]{
	let x_value:usize; let y_value:usize; let z_value:usize;
	let mut x_index:usize; let mut y_index:usize; let mut z_index:usize;
	let mut steps_index:usize;
	let mut step_direction:usize;
	let mut next_direction:usize=0;
	let mut few_steps:usize;
	let mut few_steps_index:usize;
	let mut increment_few_steps:bool;
	x_index=shell_value;
	y_index=shell_value;
	z_index=shell_value;
	steps_index=1;
	step_direction=1;
	few_steps=1;
	increment_few_steps=false;
	while(steps_index<modified_shell_sub_location)
	{
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==1 && few_steps_index<few_steps)
		{
			z_index-=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=2;
		}
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==2 && few_steps_index<few_steps)
        {
			x_index-=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=3;
		}
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==3 && few_steps_index<few_steps)
		{
			z_index+=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=4
		}
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==4 && few_steps_index<few_steps)
		{
			y_index-=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=5
		}
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==5 && few_steps_index<few_steps)
		{
			x_index+=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=6;
		}
		few_steps_index=0;
		while(steps_index<modified_shell_sub_location && step_direction==6 && few_steps_index<few_steps+1)
		{
			z_index-=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=7;
		}
		while(steps_index<modified_shell_sub_location && step_direction==7 && few_steps_index<few_steps)
		{
			y_index+=1;
			few_steps_index+=1;
			steps_index+=1;
			next_direction=2;
			increment_few_steps=true;
		}
		step_direction=next_direction;
		if(increment_few_steps==true)
		{
			few_steps+=1;
			increment_few_steps=false;
		}
	}
	x_value=x_index;
	y_value=y_index;
	z_value=z_index;
	
	return [x_value, y_value, z_value];
}
