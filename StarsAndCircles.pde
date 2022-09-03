int NUM_OF_DOT=100;
int NUM_OF_CIRCLE=5;
int NUM_OF_STAR=10;
int[][] colourArray = new int[4096][3];
Circle[] circle= new Circle[NUM_OF_CIRCLE];
Star[] star= new Star[NUM_OF_STAR];
Dot[] dot=new Dot[NUM_OF_DOT];

void setup()
{
   colourArray=initColourArray();
   size(800,800);
   for(int i=0;i<NUM_OF_DOT;i++)
   {
      dot[i]=new Dot();
   }
   for(int i=0;i<NUM_OF_CIRCLE;i++)
   {
      circle[i]=new Circle();
   }
   for(int i=0;i<NUM_OF_STAR;i++)
   {
      star[i]=new Star();
   }
} 

void draw()
{
   background(0); 
   for(int i=0;i<NUM_OF_DOT;i++)
   {
      dot[i].drawDot();
   }
   for(int i=0;i<NUM_OF_CIRCLE;i++)
   {
      circle[i].drawShape();
   }
   for(int i=0;i<NUM_OF_STAR;i++)
   {
      star[i].drawShape();
   }  
}  

//==================================

abstract class Shape implements ShapeInterface
{
   protected float a=random(1)*TWO_PI;
   protected float x=random(1)*width;  
   protected float y=random(1)*height;
   protected float r=30+random(30);
   protected float sp=1*(1+random(1));
   protected int sw=4+parseInt(random(10));
   protected int col=parseInt(random(4096));
   protected boolean directionForward=(parseInt(random(2))%2==0)?true:false;
   
   abstract void drawShapeSub();
   
   void drawShape()
   {  
      x=x+cos(a)*sp;
      y=y+sin(a)*sp;
      
      if(x<r){float aNew=((((a%TWO_PI)+(HALF_PI))*(-1))-(HALF_PI))%TWO_PI;while(aNew<0)aNew+=TWO_PI;if(aNew<=HALF_PI || aNew >=3*HALF_PI)a=aNew;}
      else if(x>width-r){float aNew=((((a%TWO_PI)+(HALF_PI))*(-1))-(HALF_PI))%TWO_PI;while(aNew<0)aNew+=TWO_PI;if(aNew>=HALF_PI && aNew <=3*HALF_PI)a=aNew;}
      if(y<r){float aNew=(a*(-1))%TWO_PI;while(aNew<0)aNew+=TWO_PI;if(aNew>=0 && aNew <=PI)a=aNew;}
      else if(y>height-r){float aNew=(a*(-1))%TWO_PI;while(aNew<0)aNew+=TWO_PI;if(aNew>=PI && aNew <=TWO_PI)a=aNew;}

      stroke(colourArray[col][0],colourArray[col][1],colourArray[col][2]);
      if(directionForward==true)
      {
         col++;
         if(col==4095){directionForward=false;};
      }
      else if(directionForward==false)
      {
         col--;
         if(col==0){directionForward=true;};
      } 
      
      drawShapeSub();
   }   
}  
//==================================

class Circle extends Shape
{ 
   void drawShapeSub()
   {
      strokeWeight(5); 
      noFill();
      ellipse(x,y,2*r,2*r);
   }  
}

//==================================
class Star extends Shape
{  
   private float ra=r;
   private float rb=r*(0.45+random(0.2));
   private int n=5+parseInt(random(3));
   private float rot=0;
   private float rotStep=(random(1.5)*TWO_PI/360)*( (parseInt(random(2))%2==0)?1:-1) ;
   
   void drawShapeSub()
   {
      strokeWeight(5); 
      noFill();
      rot=(rot+rotStep)%TWO_PI;
      beginShape();
      vertex(x+ra*cos(rot),y+ra*sin(rot));
      vertex(x+rb*cos(rot+1*PI/n),y+rb*sin(rot+1*PI/n));
      vertex(x+ra*cos(rot+2*PI/n),y+ra*sin(rot+2*PI/n));
      vertex(x+rb*cos(rot+3*PI/n),y+rb*sin(rot+3*PI/n));
      vertex(x+ra*cos(rot+4*PI/n),y+ra*sin(rot+4*PI/n));
      vertex(x+rb*cos(rot+5*PI/n),y+rb*sin(rot+5*PI/n));
      vertex(x+ra*cos(rot+6*PI/n),y+ra*sin(rot+6*PI/n));
      vertex(x+rb*cos(rot+7*PI/n),y+rb*sin(rot+7*PI/n));
      vertex(x+ra*cos(rot+8*PI/n),y+ra*sin(rot+8*PI/n));
      vertex(x+rb*cos(rot+9*PI/n),y+rb*sin(rot+9*PI/n));
      vertex(x+ra*cos(rot+10*PI/n),y+ra*sin(rot+10*PI/n)); 
      if(n>5)
      {
         vertex(x+rb*cos(rot+11*PI/n),y+rb*sin(rot+11*PI/n));
         vertex(x+ra*cos(rot+12*PI/n),y+ra*sin(rot+12*PI/n));
      }
      if(n>6)
      {
         vertex(x+rb*cos(rot+13*PI/n),y+rb*sin(rot+13*PI/n));
         vertex(x+ra*cos(rot+14*PI/n),y+ra*sin(rot+14*PI/n));
      }
      endShape(CLOSE);
   }  
}
//===========================================
interface ShapeInterface
{
   void drawShapeSub();
}  

//=========================================================================
class Dot{
   private float x0=width/2;
   private float y0=height/2;
   private float x=x0;
   private float y=y0;
   private float a=random(1)*TWO_PI;
   private float len=(sqrt((x+10)*(x+10)+(y+10)*(y+10)))*(1+random(1));
   private float dist=len*random(1);
   private float f=dist/len;
   private float sp=1*(1+random(1));  
   private float t=dist/sp;    
   private int sw;
   
   
   void drawDot()
   {
      x=x0+cos(a)*sp*(t);
      y=y0+sin(a)*sp*(t);
      dist=sqrt((x-x0)*(x-x0)+(y-y0)*(y-y0));
      f=dist/len;
      sw=(f>0.9)?9:(f>0.8)?8:(f>0.7)?7:(f>0.6)?6:(f>0.5)?5:(f>0.4)?4:(f>0.3)?3:(f>0.2)?2:(f>0.1)?1:0;
      strokeWeight(sw);
      fill(255);stroke(255);
      point(x,y);
      t++;
      if(dist>len)
      {
         t=0;
         x=x0;
         y=y0;
         len=(sqrt((x+10)*(x+10)+(y+10)*(y+10)))*(1+random(1));
         a=random(1)*TWO_PI;
         sp=1*(1+random(1));
      }       
   }  
}
//================================================================================
int[][] initColourArray()
{
  int[][] colourArray= new int[4096][3];
  int cubeLenght=16;
  int cubeVolume=cubeLenght*cubeLenght*cubeLenght;
  for(int locationIndex=1;locationIndex<=cubeVolume;locationIndex++)
  {
     int shellNumber,ringNumber, cellNumber, modifiedShellSubLocationNumber;
     int xNumber,yNumber,zNumber;
     int rNumber,gNumber,bNumber;

     shellNumber=shellRingCellVal(locationIndex)[0];
     ringNumber=shellRingCellVal(locationIndex)[1];
     cellNumber=shellRingCellVal(locationIndex)[2];
     modifiedShellSubLocationNumber=shellRingCellVal(locationIndex)[3];

     xNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[0];
     yNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[1];
     zNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[2];
     rNumber=(xNumber-1)*(cubeLenght)+(xNumber-1);
     gNumber=(yNumber-1)*(cubeLenght)+(yNumber-1);
     bNumber=(zNumber-1)*(cubeLenght)+(zNumber-1);

     colourArray[locationIndex-1]=new int[]{rNumber,gNumber,bNumber};
  }
  return colourArray;
}


  
//======================================================================================================

int[] shellRingCellVal(int location)
{
  int shellValue;
  int shellIndex;
  int ringValue;
  int ringIndex;
  int cellValue;
  int shellSubLocation;
  int shellSubLocation_index;
  int ringSubLocation;
  int previousRingValue;
  int previousRingIndex;
  int previousRingsCellsIndex;
  int previousRingsCellsValue;
  int numOfCellsPerShellValue;
  int numOfCellsPerShellIndex;
  int modifiedShellSubLocation=0;
  //---------------------
  shellIndex=1;
  while(location>(shellIndex*shellIndex*shellIndex))
  {
    shellIndex++;
  }
  shellValue=shellIndex;
  //---------------------------------------------
  shellSubLocation=location-((shellValue-1)*(shellValue-1)*(shellValue-1));
 
  ringIndex=1;
  shellSubLocation_index=1;
  while(shellSubLocation_index<shellSubLocation)
  {
    ringIndex++;
    shellSubLocation_index+=((6*ringIndex)-6);
  }
  ringValue=ringIndex;
  previousRingValue=ringValue-1;
  previousRingIndex=1;
  previousRingsCellsIndex=0;
  while(previousRingIndex<=previousRingValue)
  {
    if(previousRingIndex==1)
    {
      previousRingsCellsIndex=1;
    }
    else
    {
      previousRingsCellsIndex+=((6*previousRingIndex)-6);
    } 
    previousRingIndex++;
  }
  previousRingsCellsValue=previousRingsCellsIndex;
  ringSubLocation=shellSubLocation-previousRingsCellsValue;
  cellValue=ringSubLocation;
  //----------------------------------------------------------------
  
  numOfCellsPerShellIndex=1;
  ringIndex=1;
  while(ringIndex<shellValue)
  {
    ringIndex++;
    numOfCellsPerShellIndex+=((6*ringIndex)-6);
  }
  numOfCellsPerShellValue=numOfCellsPerShellIndex;
    
  //----------------------------------------------------------------
  
  if(shellValue%2==0)
  {modifiedShellSubLocation=shellSubLocation;}
  else if(shellValue%2==1)
  {modifiedShellSubLocation=numOfCellsPerShellValue+1-shellSubLocation;}
  //----------------------------------------------------------------
  return(new int[]{shellValue,ringValue,cellValue,modifiedShellSubLocation}); 
}
//===========================================================================

int[] xyzVal(int shellValue, int modifiedShellSubLocation)
{ 
  int xValue=0,yValue=0,zValue=0;
  int xIndex,yIndex,zIndex;
  int stepsIndex;
  int stepDirection;
  int nextDirection=0;
  int fewSteps;
  int fewStepsIndex;
  boolean incrementFewSteps;
  //---------------------------------------------
  xIndex=shellValue;
  yIndex=shellValue;
  zIndex=shellValue;
  stepsIndex=1;
  stepDirection=1;
  fewSteps=1;
  incrementFewSteps=false;
  while(stepsIndex<modifiedShellSubLocation)
  {
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==1 && fewStepsIndex<fewSteps)
    {
      zIndex-=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=2;  
    }
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==2 && fewStepsIndex<fewSteps)
    {
      xIndex-=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=3;  
    }
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==3 && fewStepsIndex<fewSteps)
    {
      zIndex+=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=4;  
    }
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==4 && fewStepsIndex<fewSteps)
    {
      yIndex-=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=5;  
    }
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==5 && fewStepsIndex<fewSteps)
    {
      xIndex+=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=6;  
    }
    fewStepsIndex=0;
    while(stepsIndex<modifiedShellSubLocation && stepDirection==6 && fewStepsIndex<fewSteps+1)
    {
      zIndex-=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=7;  
    }
    while(stepsIndex<modifiedShellSubLocation && stepDirection==7 && fewStepsIndex<fewSteps)
    {
      yIndex+=1;
      fewStepsIndex+=1;
      stepsIndex+=1;
      nextDirection=2;
      incrementFewSteps=true;
    }
    stepDirection=nextDirection;
    if(incrementFewSteps==true)
    {
       fewSteps++;
       incrementFewSteps=false;
    }
  }    
  xValue=xIndex;
  yValue=yIndex;
  zValue=zIndex; 
  
  return(new int[]{xValue,yValue,zValue});
} 
