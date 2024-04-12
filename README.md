### Particle Life

This is a simple particle life simulation. Particles are created at random positions and move in random directions. 

### Updates
Implementing quadtree for collision detection:
![Particle Life](https://github.com/harmya/particle-life/blob/main/quad.gif =200x200)

Implemented collision detection using quadtree. The quadtree is used to divide the space into smaller regions and store the particles in the regions. This allows for faster collision detection as we only need to check for collisions between particles in the same region:
![Particle Life](https://github.com/harmya/particle-life/blob/main/collision.gif =200x200)
