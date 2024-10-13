import math 

n = 1000 #number of divisions

PI = math.pi

theta1 = 1 * PI / 180
theta2 = PI

dtheta = (theta2 - theta1) / n

for i in range(501):
    z = i / 100
    E_z = 0

    for j in range(n):
        theta = theta1 + (j + 0.5) * dtheta 
        dE = 2 * PI * (math.sin(theta) * (z-math.cos(theta))) / math.pow((z*z-2*z*math.cos(theta)+1), 1.5) * dtheta
        E_z += dE

    print(E_z)
