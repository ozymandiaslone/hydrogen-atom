#### Have you ever wanted to computationally solve the Schrödinger Equation of a Hydrogen Atom?
### No?
##### That's probably normal.

### The Goal
Let's set the stage: the end goal of this project is to generate visualization of the Electron "Orbitals" which many people will be familiar with from classes such as highschool chemistry.
![Here is a video of what the program currently visualizes.](./hydrogen-demo0.gif)
^^ this GIF is kinda messed up low FPS, but you get the idea...


#### A (very) quick crashcourse in Quantum Mechanics
Firstly, I should be very clear that I am NOT a physicist, meaning that this explanation I am about to give is based entirely on my own understanding of this subject (that likely has gaps) which I researched in order to complete this project :)

Basically, this Schrödinger Equation we are working with describes the state of ONE electron interacting with ONE proton. AKA, a hydrogen atom. 
Luckily for us, really intelligent Quantum Mechanicists have discovered that this is one of the few cases wherein the Schrödinger Equation can be analytically solved. If we make sure to use polar coordinates, we can 
separate the equation into two parts: a radial equation, and a spherical harmonic. This Radial bit makes use of a family of equations known as 'Associated Laguerre Polynomails', and only depends on the distance from the origin. The Spherical Harmonics, which only depend on the angular coordinates, are well-known mathematical equations. 


