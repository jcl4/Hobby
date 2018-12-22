# Notes taken during learning about uniform buffers

## Todo for impimentation
Final goal is to have the colored quad transformed statically via a model matrix passed to the vertex shader. The Transform will be set once during model build up. Updating of the transform each frame to be added will be next feature to be added.

- [ ] Create Descriptor set pool
- [ ] Model passes data to material type to build descriptor set



* All this work will be done under the uniform buffers branch

## Notes:
### Descriptor
* A [Descriptor][1] is a binding for data stored in a GPU buffer used in shaders:
    * layout(set = 0, binding = 0) is a descriptor to set 0 binding 0

### Descriptor Sets
* [Descriptor sets][2] contain the bindings
    * Descriptor sets are used in the [`draw_index`][3] method when building the command buffer
* Descriptor sets come from the descriptor set pool
* Descriptor sets corespond to the set numbers in the shader, the set id number is set during creation of the set pool
* Note sure about persistent vs fixed size
    * Vulkano teapot tutorial uses persistent descriptor set
    * Airjump used fixed size descriptor set pool 
    * Jakar engine uses fixed size pool

* Note from Vulkano Docs on fixed size descriptor sets: _You are encouraged to use this type when you need a different descriptor set at each frame, or regularly during the execution._
* Graphics Pool needs a graphics pipeline to create
    * Currently pipelines are generated at the model level
    * for the time being each model will get a pool based on the material type used in the model


### Descriptor set and pool creation
* `let pool = FixedSizeDescriptorSetsPool::new(graphics_pipeline.clone(), 0);` - [doc][4]

* `let set = pool.next().add_buffer(buffer).unwrap().build();` - [doc][5]
    * many `add_buffer` calls can be made and the order they are called corresponds to the binding number in the shader

### Buffer creation
* create the buffer as a `CpuBufferPool`
* use shader mod for type: `CpuBufferPool::<vs::ty::Data>::new(
        device.clone(),
        vulkano::buffer::BufferUsage::all(),
    );`

[1]: https://docs.rs/vulkano/0.10.0/vulkano/descriptor/index.html
[2]: https://docs.rs/vulkano/0.10.0/vulkano/descriptor/descriptor_set/index.html
[3]: https://docs.rs/vulkano/0.10.0/vulkano/command_buffer/struct.AutoCommandBufferBuilder.html#method.draw_indexed
[4]: https://docs.rs/vulkano/0.10.0/vulkano/descriptor/descriptor_set/struct.FixedSizeDescriptorSetsPool.html#method.new
[5]: https://docs.rs/vulkano/0.10.0/vulkano/descriptor/descriptor_set/struct.FixedSizeDescriptorSetBuilder.html