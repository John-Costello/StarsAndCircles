import pygame, math, random, sys
from pygame.locals import *

FPS=60
WINDOWWIDTH=800
WINDOWHEIGHT=800
BLACK=(0,0,0)
NUM_OF_DOT=100
NUM_OF_CIRCLE=5
NUM_OF_STAR=10
PI=math.pi
HALF_PI=math.pi/2
TWO_PI=math.pi*2
global colourArray
#===============================================
def setup():
    global colourArray
    colourArray=initColourArray()
    global dots
    dots=[Dot()]
    for i in range(1,NUM_OF_DOT):
        dots.append(Dot())
    global circles
    circles=[Circle()]
    for i in range(1,NUM_OF_CIRCLE):
        circles.append(Circle())    
    global stars
    stars=[Star()]
    for i in range(1,NUM_OF_STAR):
        stars.append(Star())    
#===============================================
def main():
    global MAINCLOCK, SCREEN
    pygame.init()
    SCREEN=pygame.display.set_mode((WINDOWWIDTH, WINDOWHEIGHT))
    SCREEN.fill(BLACK)
    pygame.display.set_caption("Stars and Circles")
    pygame.display.update()
    MAINCLOCK=pygame.time.Clock()
    setup()
    running=True
    while(running):
        for event in pygame.event.get():
            if(event.type==pygame.QUIT):
                running=False
        SCREEN.fill(BLACK)
        if running==True:
            for i in range(NUM_OF_DOT):
                dots[i].drawDot()
            for i in range(NUM_OF_CIRCLE):    
                circles[i].drawShape()
            for i in range(NUM_OF_STAR):
                stars[i].drawShape()                
            MAINCLOCK.tick(FPS)
            pygame.display.update()
    pygame.quit()
    sys.exit()

#===============================================
class Shape:
    def shapeInit(self):
        self.a=random.random()*2*math.pi
        self.x=random.random()*WINDOWWIDTH
        self.y=random.random()*WINDOWHEIGHT
        self.r=30+random.random()*30
        self.sp=1*(1+random.random())
        self.sw=4+math.floor(10*random.random())
        self.col=math.floor(4096*random.random())
        self.directionForward=True if (random.random()<0.5) else False
    
    def drawShapeSuper(self):
        self.x=self.x+math.cos(self.a)*self.sp
        self.y=self.y+math.sin(self.a)*self.sp
        
        if(self.x<self.r):
            aNew=((((self.a%(TWO_PI))+(HALF_PI))*(-1))-(HALF_PI))%(TWO_PI)
            while(aNew<0):
                aNew+=(TWO_PI)
            if(aNew<=PI or aNew>3*HALF_PI):
                self.a=aNew
        elif(self.x>WINDOWWIDTH-self.r):
            aNew=((((self.a%(TWO_PI))+(HALF_PI))*(-1))-(HALF_PI))%(TWO_PI)
            while(aNew<0):
                aNew+=(TWO_PI)
            if(aNew>=((HALF_PI)) and aNew<=(3*HALF_PI)):
                self.a=aNew
        if(self.y<self.r):
            aNew=(self.a*(-1))%(TWO_PI)
            while(aNew<0):
                aNew+=(TWO_PI)
            if(aNew>0 and aNew<=PI):
                self.a=aNew
        elif(self.y>WINDOWHEIGHT-self.r):
            aNew=(self.a*(-1))%(TWO_PI)
            while(aNew<0):
                aNew+=(TWO_PI)
            if(aNew>=PI and aNew<=(TWO_PI)):
                self.a=aNew

        if(self.directionForward==True):
            self.col+=1
            if(self.col==4095):
                self.directionForward=False
        elif(self.directionForward==False):
            self.col-=1
            if(self.col==0):
                self.directionForward=True
               
#===============================================    
class Circle(Shape):

    def __init__(self):
        self.shapeInit()
        
    def drawShape(self):
       self.drawShapeSuper()
       pygame.draw.circle(SCREEN,colourArray[self.col],(self.x,self.y),self.r,5)
       pass
#===============================================
class Star(Shape):

    def __init__(self):
        self.shapeInit()
        self.ra=self.r
        self.rb=self.r*(0.45+0.2*random.random())
        self.n=5+int(3*random.random())
        self.rot=0
        self.rotStep=(1.5*random.random()*TWO_PI/360)*(1 if (random.random()<0.5) else -1)

    def drawShape(self):
        self.drawShapeSuper()
        self.rot=(self.rot+self.rotStep)%TWO_PI
        x=self.x
        y=self.y
        rot=self.rot
        ra=self.ra
        rb=self.rb
        n=self.n
        vertices=[(x+ra*math.cos(rot), y+ra*math.sin(rot))]
        vertices.append((x+rb*math.cos(rot+1*PI/n), y+rb*math.sin(rot+1*PI/n)))
        vertices.append((x+ra*math.cos(rot+2*PI/n), y+ra*math.sin(rot+2*PI/n)))
        vertices.append((x+rb*math.cos(rot+3*PI/n), y+rb*math.sin(rot+3*PI/n)))
        vertices.append((x+ra*math.cos(rot+4*PI/n), y+ra*math.sin(rot+4*PI/n)))
        vertices.append((x+rb*math.cos(rot+5*PI/n), y+rb*math.sin(rot+5*PI/n)))
        vertices.append((x+ra*math.cos(rot+6*PI/n), y+ra*math.sin(rot+6*PI/n)))
        vertices.append((x+rb*math.cos(rot+7*PI/n), y+rb*math.sin(rot+7*PI/n)))
        vertices.append((x+ra*math.cos(rot+8*PI/n), y+ra*math.sin(rot+8*PI/n)))
        vertices.append((x+rb*math.cos(rot+9*PI/n), y+rb*math.sin(rot+9*PI/n)))
        vertices.append((x+ra*math.cos(rot+10*PI/n), y+ra*math.sin(rot+10*PI/n)))
        if(n>5):
           vertices.append((x+rb*math.cos(rot+11*PI/n), y+rb*math.sin(rot+11*PI/n)))
           vertices.append((x+ra*math.cos(rot+12*PI/n), y+ra*math.sin(rot+12*PI/n)))
        if(n>6):
           vertices.append((x+rb*math.cos(rot+13*PI/n), y+rb*math.sin(rot+13*PI/n)))
           vertices.append((x+ra*math.cos(rot+14*PI/n), y+ra*math.sin(rot+14*PI/n)))
        pygame.draw.polygon(SCREEN,colourArray[self.col],vertices,5) 
#===============================================
class Dot:

    def __init__(self):
        self.x=x0=WINDOWWIDTH/2
        self.y=y0=WINDOWHEIGHT/2
        self.a=random.random()*TWO_PI
        self.len=(math.sqrt((self.x+10)**2+(self.y+10)**2))*(1+random.random())
        self.dist=self.len*random.random()
        self.f=self.dist/self.len
        self.sp=1*(1+random.random())
        self.t=self.dist/self.sp

    def drawDot(self):
        x0=WINDOWWIDTH/2
        y0=WINDOWHEIGHT/2
        a=self.a
        sp=self.sp
        t=self.t
        self.x=x0+math.cos(a)*sp*t
        self.y=y0+math.sin(a)*sp*t
        self.dist=math.sqrt((self.x-x0)**2+(self.y-y0)**2)
        self.f=self.dist/self.len
        self.sw=9 if (self.f>0.9) else 8 if (self.f>0.8) else 7 if (self.f>0.7) else\
                6 if (self.f>0.6) else 5 if (self.f>0.5) else 4 if (self.f>0.4) else\
                3 if (self.f>0.3) else 2 if (self.f>0.2) else 1 if (self.f>0.1) else 0
        if(self.sw==1):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,1,1],self.sw)
        if(self.sw==2):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,2,1],self.sw)
        if(self.sw==3):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,2,2],self.sw)
        if(self.sw==4):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,3,2],self.sw)
        if(self.sw==5):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,3,3],self.sw)
        if(self.sw==6):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,4,3],self.sw)
        if(self.sw==7):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,4,4],self.sw)
        if(self.sw==8):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,5,4],self.sw)
        if(self.sw==9):pygame.draw.ellipse(SCREEN,(255,255,255),[self.x,self.y,5,5],self.sw)
        self.t+=1
        if(self.dist>self.len):
            self.t=0
            self.x=x0
            self.y=y0
            self.len=(math.sqrt((self.x+10)**2+(self.y+10)**2))*(1+random.random())
            self.a=random.random()*TWO_PI
            self.sp=1*(1+random.random())
#===============================================  
def initColourArray():
    setColourArray=[]   
    cubeLength=16
    cubeVolume=cubeLength**3
    for locationIndex in range(1,4097,1):
        shellNumber=shellRingCellVal(locationIndex)[0]
        ringNumber=shellRingCellVal(locationIndex)[1]
        cellNumber=shellRingCellVal(locationIndex)[2]
        modifiedShellSubLocationNumber=shellRingCellVal(locationIndex)[3]
        xNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[0]
        yNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[1]
        zNumber=xyzVal(shellNumber,modifiedShellSubLocationNumber)[2]
        rNumber=(xNumber-1)*(cubeLength)+(xNumber-1)
        gNumber=(yNumber-1)*(cubeLength)+(yNumber-1)
        bNumber=(zNumber-1)*(cubeLength)+(zNumber-1)
        setColourArray.append((rNumber,gNumber,bNumber))
    return setColourArray
#===============================================
def shellRingCellVal(location):
    modifiedShellSubLocation=0
    shellIndex=1
    while(location>(shellIndex**3)):
        shellIndex+=1
    shellValue=shellIndex
    #--------------------------------------
    shellSubLocation=location-((shellValue-1)**3)
    ringIndex=1
    shellSubLocation_index=1
    while(shellSubLocation_index<shellSubLocation):
        ringIndex+=1
        shellSubLocation_index+=((6*ringIndex)-6)
    ringValue=ringIndex
    previousRingValue=ringValue-1
    previousRingIndex=1
    previousRingsCellsIndex=0
    while(previousRingIndex<=previousRingValue):
        if(previousRingIndex==1):previousRingsCellsIndex=1
        else: previousRingsCellsIndex+=((6*previousRingIndex)-6)
        previousRingIndex+=1
    previousRingsCellsValue=previousRingsCellsIndex
    ringSubLocation=shellSubLocation-previousRingsCellsValue
    cellValue=ringSubLocation
    #--------------------------------------
    numOfCellsPerShellIndex=1
    ringIndex=1;
    while(ringIndex<shellValue):
        ringIndex+=1
        numOfCellsPerShellIndex+=((6*ringIndex)-6)
    numOfCellsPerShellValue=numOfCellsPerShellIndex
    #--------------------------------------
    if(shellValue%2==0):
        modifiedShellSubLocation=shellSubLocation
    elif(shellValue%2==1):
        modifiedShellSubLocation=numOfCellsPerShellValue+1-shellSubLocation
    #--------------------------------------
    return((shellValue,ringValue,cellValue,modifiedShellSubLocation))
#===============================================
def xyzVal(shellValue, modifiedShellSubLocation):
    xValue=0
    yValue=0
    zValue=0
    nextDirection=0
    xIndex=shellValue
    yIndex=shellValue
    zIndex=shellValue
    stepsIndex=1
    stepDirection=1
    fewSteps=1
    incrementFewSteps=False
    while(stepsIndex<modifiedShellSubLocation):
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==1 and fewStepsIndex<fewSteps):
            zIndex-=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=2
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==2 and fewStepsIndex<fewSteps):
            xIndex-=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=3
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==3 and fewStepsIndex<fewSteps):
            zIndex+=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=4
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==4 and fewStepsIndex<fewSteps):
            yIndex-=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=5
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==5 and fewStepsIndex<fewSteps):
            xIndex+=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=6
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==6 and fewStepsIndex<fewSteps+1):
            zIndex-=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=7
        fewStepsIndex=0
        while(stepsIndex<modifiedShellSubLocation and stepDirection==7 and fewStepsIndex<fewSteps):
            yIndex+=1
            fewStepsIndex+=1
            stepsIndex+=1
            nextDirection=2
            incrementFewSteps=True
        fewStepsIndex=0
        stepDirection=nextDirection
        if(incrementFewSteps==True):
            fewSteps+=1
            incrementFewSteps=False
    xValue=xIndex
    yValue=yIndex
    zValue=zIndex
    return((xValue,yValue,zValue))
#===============================================
if __name__=='__main__':
    main()
