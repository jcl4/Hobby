# Hobby Render Pipelines
## Colored Mesh Pipeline
### Writen 11/6/19 - James Longino
- `Renderer` holds a `Option<ColoredMeshPipeline>`
    - Created as `None` when `Renderer::new()` called
- Pipeline is created in binary: `ColoredMeshPipeline::new()`
- Models are part of the pipeline
  - `pipeline.add_model(model)`
- Models are created with vertices and indices
  - Verticies are `Vec<ColoredMeshVertex>`
  - Indicies are `Vec<u16>`
- Pipeline is then sent to application when started
- Application adds it to the renderer
- Renderer Builds the pipeline
- Example usage (see triangle example)
    ``` Rust
        let window_settings = WindowSettings::default();

        let vertices = vec![
            ColoredMeshVertex::new([0.0, 0.5, 0.0], [1.0, 0.0, 0.0, 1.0]),
            ColoredMeshVertex::new([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0, 1.0]),
            ColoredMeshVertex::new([0.5, -0.5, 0.0], [0.0, 0.0, 1.0, 1.0]),
        ];
        let indices: Vec<u16> = vec![0, 1, 2];
        let triangle_model = ColoredMeshModel::new(vertices, indices);
        let mut pipeline = ColoredMeshPipeline::new();
        pipeline.add_model(triangle_model);

        let app = Application::new(window_settings);
        app.start(pipeline);
    ```
  
