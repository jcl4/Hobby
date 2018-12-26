# Notes on development of Hobby rendering project

## Mesh and Model API Design
- a model holds a mesh and materials
- a material is an enum defining a pipeline for the mesh
    - basic
    - textured
    - phong
    - bpr
    - wireframe
- vertex will have position as the only required member and all other memebers will be optional
    - required vertex data will be determined by material type and needs to be checked
- A model contains a tranform
    - A model will also have children models, where transforms are relative to the parent
- A model will need to impliment the model update trait, this gives an update method that can be used to update the model

- Once a model is added to the game it is built
    - this creates all the vulkan resources required to render the model
    - each model has a pipeline, descirptor set and pool required 

- A mesh will hold the mesh data (vertices and indices) and the vertex and index buffers

## Model Update functionality
