# Model & Pipeline API
- Notes on how to put together a model based pipeline

## Modle Object
- Trait Based
  - Update function
  - draw function
- Holds its own buffers
  - Vertex, Index, Uniform
- Holds a value of Pipeline enum

## Pipeline Object
- Trait Based
- 

## Pipeline Enum
- Defines the different type of enums
  - Colored Mesh 
  - Textured 


## Application holds vector of models
- calls update on each
- passes vector to renderer for drawing