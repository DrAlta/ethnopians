from scipy.optimize import leastsq
import numpy as np
#import matplotlib.pyplot as plt
import math

def main():
   # data provided
   x=np.array([1.0, 10, 100, 1000, 1000, 1000, 1001, 1100])
   y=np.array([1.0, 10, 100,    1,  100, 1000,    1,  100])
   z = np.zeros(len(x))
   for i in range(len(x)) :
       z[i] = math.sqrt((x[i]*x[i])+(y[i]*y[i]))
   # here, create lambda functions for Line, Quadratic fit
   # tpl is a tuple that contains the parameters of the fit
   def danja( tpl,ax, ay) :
       miny = np.minimum(ax, ay)
       maxy = np.maximum(ax, ay)
       diff = maxy - miny
       q = maxy + (miny * (1.0 / maxy))
       min_c = (miny * (tpl[0] + (diff * tpl[1])))
       max_c = (maxy * (tpl[2] + (diff * tpl[3])))
       c = min_c + max_c
       return (( q + miny + maxy) / 2.0 ) + c
   # ErrorFunc is the diference between the danja and the y "experimental" data
   ErrorFunc=lambda tpl,x,y,z: danja(tpl,x,y)-z
   #tplInitial contains the "first guess" of the parameters 
   tplInitial1=(-0.513932022 * 0.5,0.43 * 0.5, -0.04546304 * 0.5 , -0.004 * 0.5 )
   print("inital guess:", tplInitial1)
   # leastsq finds the set of parameters in the tuple tpl that minimizes
   # ErrorFunc=yfit-yExperimental
   tplFinal1,success=leastsq(ErrorFunc,tplInitial1[:],args=(x,y,z))
   print (" linear fit ",tplFinal1)
   xx1=np.linspace(x.min(),x.max(),50)
   print("err:")
   print(danja(tplFinal1,x, y) - z)
   #plt.plot(xx1,yy1,'r-',x,y,'bo',xx2,yy2,'g-')
   #plt.show()

if __name__=="__main__":
   main()