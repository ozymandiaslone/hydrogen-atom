#### Have you ever wanted to computationally solve the Schrödinger Equation of a Hydrogen Atom?
### No?
##### That's probably normal.

### The Goal
Let's set the stage: the end goal of this project is to generate visualizations of the Electron "Orbitals" which many people will be familiar with from classes such as highschool chemistry.
![Here is a video of what the program currently visualizes.](./hydrogen-demo0.gif)
^^ this GIF is kinda messed up with really low FPS for some reason, but you get the idea...


#### A (very) quick crashcourse in Quantum Mechanics
Firstly, I should be very clear that I am NOT a physicist, meaning that this explanation I am about to give is based entirely on my own understanding of this subject (that likely has gaps) which I researched in order to complete this project :)

Basically, this Schrödinger Equation we are working with describes the state of ONE electron interacting with ONE proton. AKA, a hydrogen atom. 
Luckily for us, really intelligent Quantum Physicists have discovered that this is one of the few cases wherein the Schrödinger Equation can be analytically solved. If we make sure to use spherical coordinates, we can 
separate the equation into two parts: radial and angular. The Radial bit makes use of a family of equations known as 'Associated Laguerre Polynomials', and only depends on the distance from the origin. The angular part makes use of Spherical Harmonics, which are a set of well-known mathematical equations. 

Spherical harmonics describe oscillations on the surface of a sphere, analogous to how sine and cosine describe oscillations along a line. They are constructed from associated Legendre polynomials for the polar angle and sinusoidal functions for the azimuthal angle.


