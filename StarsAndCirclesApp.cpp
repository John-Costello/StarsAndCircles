#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"


using namespace ci;
using namespace ci::app;
using namespace std;
const float PI = 3.14159265;
const float TWO_PI =  6.2831853;
const float HALF_PI = 1.5707963;
const int NUM_OF_DOTT = 100;
const int NUM_OF_CIRCLE = 5;
const int NUM_OF_STAR = 10;
const int WINDOW_WIDTH = 800;
const int WINDOW_HEIGHT = 800;
static float random(float x)
{
	return static_cast <float> (rand()) / (static_cast <float> (RAND_MAX / (x)));
}
static int castInt(float x)
{
	return (int)x;
}
static int* shellRingCellVal(int location);
static int* xyzVal(int shellValue, int modifiedShellSubLocation);
static int** initializeColourArray();
static int** colourArray = initializeColourArray();

//=======================================================

class Dott
{
public:
	float x0;
	float y0;
	float x;
	float y;
	float a;
	float len; 
	float dist;
	float f;
	float sp;
	float t;
	float sw=1;
	//=======================================================
	Dott::Dott()
	{
		x0 = ((float)WINDOW_WIDTH) / 2;  
		y0 = ((float)WINDOW_HEIGHT) / 2; 
		x = x0;
		y = y0;
		a = (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)) * TWO_PI;
		len = (sqrt((x + 10) * (x + 10) + (y + 10) * (y + 10))) * (1 + (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)));
		dist = len * (static_cast <float> (rand()) / static_cast <float> (RAND_MAX));
		f = dist / len;
		sp = 1 * (1 + (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)));
		t = dist / sp;
	}
	void drawDott()
	{
		x = x0 + cos(a) * sp * (t);
		y = y0 + sin(a) * sp * (t);
		dist = sqrt((x - x0) * (x - x0) + (y - y0) * (y - y0));
		f = dist / len;
		sw = (f > 0.9) ? 9 : (f > 0.8) ? 8 : (f > 0.7) ? 7 : (f > 0.6) ? 6 : (f > 0.5) ? 5 : (f > 0.4) ? 4 : (f > 0.3) ? 3 : (f > 0.2) ? 2 : (f > 0.1) ? 1 : 0;
		if (f > 0.1) { gl::color(Color(1.f, 1.f, 1.f)); }
		else{ gl::color(Color(0.f, 0.f, 0.f)); }
		
		gl::drawSolidCircle(vec2(x, y), ((sw<1.8)?sw:sw/1.8) );
		t++;
		if (dist > len)
		{
			t = 0;
			x = x0;
			y = y0;
			len = (sqrt((x + 10) * (x + 10) + (y + 10) * (y + 10))) * (1 + (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)));
			a = (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)) * TWO_PI;
			sp = 1 * (1 + (static_cast <float> (rand()) / static_cast <float> (RAND_MAX)));
		}
	}
};
//=======================================================
class Shape 
{
protected:
    float a = random(1) * TWO_PI;
    float aNew;
    float x = random(1) * WINDOW_WIDTH;
    float y = random(1) * WINDOW_HEIGHT;
    float r = 30 + random(30);
    float sp = 1 * (1 + random(1));
    int sw = 4 + castInt(random(10));
    int col = castInt(random(4095));
    bool directionForward = (castInt(random(2)) % 2 == 0) ? true : false;
public:
    virtual void drawShapeSub() {};

    void drawShape()
    {
        x = x + cos(a) * sp;
        y = y + sin(a) * sp;

        if (x < r) { aNew = fmod((((fmod(a, TWO_PI) + (HALF_PI)) * (-1)) - (HALF_PI)), TWO_PI); while (aNew < 0)aNew += TWO_PI; if (aNew <= HALF_PI || aNew >= 3 * HALF_PI)a = aNew; }
        else if (x > WINDOW_WIDTH - r) { aNew = fmod((((fmod(a, TWO_PI) + (HALF_PI)) * (-1)) - (HALF_PI)), TWO_PI); while (aNew < 0)aNew += TWO_PI; if (aNew >= HALF_PI && aNew <= 3 * HALF_PI)a = aNew; }
        if (y < r) { aNew = fmod((a * (-1)), TWO_PI); while (aNew < 0)aNew += TWO_PI; if (aNew >= 0 && aNew <= PI)a = aNew; }
        else if (y > WINDOW_HEIGHT - r) { aNew = fmod((a * (-1)), TWO_PI); while (aNew < 0)aNew += TWO_PI; if (aNew >= PI && aNew <= TWO_PI)a = aNew; }
        gl::color(colourArray[col][0]/255., colourArray[col][1]/255., colourArray[col][2]/255.);
        if (directionForward == true)
        {
            col++;
            if (col == 4095) { directionForward = false; };
        }
        else if (directionForward == false)
        {
            col--;
            if (col == 0) { directionForward = true; };
        }

        drawShapeSub();
    }
};
//=======================================================
class Circle : public Shape
{
private:
    void drawShapeSub()
    {        
        gl::drawStrokedCircle(vec2(x, y), r, 5, -1);
    }
};
//=======================================================

class Star : public Shape
{
private:
    float ra = r;
	float rb = r * (0.45 + random(0.2));
	int n = 5 + castInt(random(3));
	float rot = 0;
	float rotStep = (random(1.5) * TWO_PI / 360) * ((castInt(random(2)) % 2 == 0) ? 1 : -1);

	void drawShapeSub()
	{
        gl::lineWidth(5);
        
		rot = fmod((rot + rotStep), TWO_PI);
        Path2d starPath;
		starPath.moveTo(vec2(x + ra * cos(rot), y + ra * sin(rot)));
        starPath.lineTo(vec2(x + rb * cos(rot + 1 * PI / n), y + rb * sin(rot + 1 * PI / n)));
        starPath.lineTo(vec2(x + ra * cos(rot + 2 * PI / n), y + ra * sin(rot + 2 * PI / n)));
        starPath.lineTo(vec2(x + rb * cos(rot + 3 * PI / n), y + rb * sin(rot + 3 * PI / n)));
        starPath.lineTo(vec2(x + ra * cos(rot + 4 * PI / n), y + ra * sin(rot + 4 * PI / n)));
        starPath.lineTo(vec2(x + rb * cos(rot + 5 * PI / n), y + rb * sin(rot + 5 * PI / n)));
        starPath.lineTo(vec2(x + ra * cos(rot + 6 * PI / n), y + ra * sin(rot + 6 * PI / n)));
        starPath.lineTo(vec2(x + rb * cos(rot + 7 * PI / n), y + rb * sin(rot + 7 * PI / n)));
        starPath.lineTo(vec2(x + ra * cos(rot + 8 * PI / n), y + ra * sin(rot + 8 * PI / n)));
        starPath.lineTo(vec2(x + rb * cos(rot + 9 * PI / n), y + rb * sin(rot + 9 * PI / n)));
        starPath.lineTo(vec2(x + ra * cos(rot + 10 * PI / n), y + ra * sin(rot + 10 * PI / n)));
		if (n > 5)
		{
            starPath.lineTo(vec2(x + rb * cos(rot + 11 * PI / n), y + rb * sin(rot + 11 * PI / n)));
            starPath.lineTo(vec2(x + ra * cos(rot + 12 * PI / n), y + ra * sin(rot + 12 * PI / n)));
		}
		if (n > 6)
		{
            starPath.lineTo(vec2(x + rb * cos(rot + 13 * PI / n), y + rb * sin(rot + 13 * PI / n)));
            starPath.lineTo(vec2(x + ra * cos(rot + 14 * PI / n), y + ra * sin(rot + 14 * PI / n)));
		}
        
		gl::draw(starPath);       
	}
};


//=======================================================
Dott* dott = new Dott[NUM_OF_DOTT]; 
Circle* circle = new Circle[NUM_OF_CIRCLE];
Star* star = new Star[NUM_OF_STAR];
//=======================================================
class StarsAndCirclesApp : public App {
  public:
	
	static  void prepareSettings(Settings* settings);
	void draw() override;
};

void StarsAndCirclesApp::prepareSettings(Settings *settings)
{
	settings->setWindowSize(WINDOW_WIDTH, WINDOW_HEIGHT);
	settings->setTitle("Stars and Circles");
	settings->setFrameRate(60);
}

void StarsAndCirclesApp::draw()
{
	gl::clear( Color( 0, 0, 0 ) ); 
	for (int i = 0; i < NUM_OF_DOTT; i++)
	{
		dott[i].drawDott();
	}
    for (int i = 0; i < NUM_OF_CIRCLE; i++)
    {
        circle[i].drawShape();
    }
    for (int i = 0; i < NUM_OF_STAR; i++)
    {
        star[i].drawShape();
    }
}
//=======================================================

CINDER_APP( StarsAndCirclesApp, RendererGl, &StarsAndCirclesApp::prepareSettings)

//============================================================================
static int* shellRingCellVal(int location)
{
    static int shellValue;
    static int shellIndex;
    static int ringValue;
    static int ringIndex;
    static int cellValue;
    static int shellSubLocation;
    static int shellSubLocation_index;
    static int ringSubLocation;
    static int previousRingValue;
    static int previousRingIndex;
    static int previousRingsCellsIndex;
    static int previousRingsCellsValue;
    static int numOfCellsPerShellValue;
    static int numOfCellsPerShellIndex;
    static int modifiedShellSubLocation = 0;
    //---------------------
    shellIndex = 1;
    while (location > (shellIndex * shellIndex * shellIndex))
    {
        shellIndex++;
    }
    shellValue = shellIndex;
    //---------------------------------------------
    shellSubLocation = location - ((shellValue - 1) * (shellValue - 1) * (shellValue - 1));

    ringIndex = 1;
    shellSubLocation_index = 1;
    while (shellSubLocation_index < shellSubLocation)
    {
        ringIndex++;
        shellSubLocation_index += ((6 * ringIndex) - 6);
    }
    ringValue = ringIndex;
    previousRingValue = ringValue - 1;
    previousRingIndex = 1;
    previousRingsCellsIndex = 0;
    while (previousRingIndex <= previousRingValue)
    {
        if (previousRingIndex == 1)
        {
            previousRingsCellsIndex = 1;
        }
        else
        {
            previousRingsCellsIndex += ((6 * previousRingIndex) - 6);
        }
        previousRingIndex++;
    }
    previousRingsCellsValue = previousRingsCellsIndex;
    ringSubLocation = shellSubLocation - previousRingsCellsValue;
    cellValue = ringSubLocation;
    //----------------------------------------------------------------

    numOfCellsPerShellIndex = 1;
    ringIndex = 1;
    while (ringIndex < shellValue)
    {
        ringIndex++;
        numOfCellsPerShellIndex += ((6 * ringIndex) - 6);
    }
    numOfCellsPerShellValue = numOfCellsPerShellIndex;

    //----------------------------------------------------------------

    if (shellValue % 2 == 0)
    {
        modifiedShellSubLocation = shellSubLocation;
    }
    else if (shellValue % 2 == 1)
    {
        modifiedShellSubLocation = numOfCellsPerShellValue + 1 - shellSubLocation;
    }
    //----------------------------------------------------------------
    static int* srcmValuesArray = new int[4];
    srcmValuesArray[0] = shellValue;
    srcmValuesArray[1] = ringValue;
    srcmValuesArray[2] = cellValue;
    srcmValuesArray[3] = modifiedShellSubLocation;
    return(srcmValuesArray);
}
//============================================================================
static int* xyzVal(int shellValue, int modifiedShellSubLocation)
{
    static int xValue = 0, yValue = 0, zValue = 0;
    static int xIndex, yIndex, zIndex;
    static int stepsIndex;
    static int stepDirection;
    static int nextDirection = 0;
    static int fewSteps;
    static int fewStepsIndex;
    static bool incrementFewSteps;
    //---------------------------------------------
    xIndex = shellValue;
    yIndex = shellValue;
    zIndex = shellValue;
    stepsIndex = 1;
    stepDirection = 1;
    fewSteps = 1;
    incrementFewSteps = false;
    while (stepsIndex < modifiedShellSubLocation)
    {
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 1 && fewStepsIndex < fewSteps)
        {
            zIndex -= 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 2;
        }
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 2 && fewStepsIndex < fewSteps)
        {
            xIndex -= 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 3;
        }
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 3 && fewStepsIndex < fewSteps)
        {
            zIndex += 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 4;
        }
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 4 && fewStepsIndex < fewSteps)
        {
            yIndex -= 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 5;
        }
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 5 && fewStepsIndex < fewSteps)
        {
            xIndex += 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 6;
        }
        fewStepsIndex = 0;
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 6 && fewStepsIndex < fewSteps + 1)
        {
            zIndex -= 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 7;
        }
        while (stepsIndex < modifiedShellSubLocation && stepDirection == 7 && fewStepsIndex < fewSteps)
        {
            yIndex += 1;
            fewStepsIndex += 1;
            stepsIndex += 1;
            nextDirection = 2;
            incrementFewSteps = true;
        }
        stepDirection = nextDirection;
        if (incrementFewSteps == true)
        {
            fewSteps++;
            incrementFewSteps = false;
        }
    }
    xValue = xIndex;
    yValue = yIndex;
    zValue = zIndex;

    static int* xyzValuesArray = new int[3];
    xyzValuesArray[0] = xValue;
    xyzValuesArray[1] = yValue;
    xyzValuesArray[2] = zValue;
    return(xyzValuesArray);
}
//============================================================================

static int** initializeColourArray()
{
    static int** colourArray = new int* [4096]; 
    for (static int i = 0; i < 4096; i++)
    {
        colourArray[i] = new int[3];
        for (static int j = 0; j < 3; j++)
        {
            colourArray[i][j] = 125;
        }
    }
    static int cubeLenght = 16;
    static int cubeVolume = cubeLenght * cubeLenght * cubeLenght;
    for (static int locationIndex = 1; locationIndex <= cubeVolume; locationIndex++)
    {
        static int shellNumber, ringNumber, cellNumber, modifiedShellSubLocationNumber;
        static int xNumber, yNumber, zNumber;
        static int rNumber, gNumber, bNumber;

        shellNumber = shellRingCellVal(locationIndex)[0];
        ringNumber = shellRingCellVal(locationIndex)[1];
        cellNumber = shellRingCellVal(locationIndex)[2];
        modifiedShellSubLocationNumber = shellRingCellVal(locationIndex)[3];

        xNumber = xyzVal(shellNumber, modifiedShellSubLocationNumber)[0];
        yNumber = xyzVal(shellNumber, modifiedShellSubLocationNumber)[1];
        zNumber = xyzVal(shellNumber, modifiedShellSubLocationNumber)[2];
        rNumber = (xNumber - 1) * (cubeLenght)+(xNumber - 1);
        gNumber = (yNumber - 1) * (cubeLenght)+(yNumber - 1);
        bNumber = (zNumber - 1) * (cubeLenght)+(zNumber - 1);

        colourArray[locationIndex - 1] = new int[3] {rNumber, gNumber, bNumber};
    }
    return colourArray;
}


