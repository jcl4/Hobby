# TODO for hobby project

## Next
- [x] Model Update Capability
- [x] Build out Transform functionality
    - [x] Default Transform
    - [x] translate function
    - [x] rotate function
- [ ] Camera - Quaternion Based
- [x] Fix max frame time reporting
- [ ] GLTF import


## Big Tasks - to work eventually
- [ ] Enable wire-frame rendering: see rasterizer in pipeline creation
- [ ] Depth buffering
- [ ] Push Constants
- [ ] Lighting
- [ ] Input management
- [ ] Color see [here][L1]
    - [ ] Update Vertex to use color struct
    - [ ] Update clear color
- [ ] Switch to ECS architecture
- [ ] Refactor frame timer into generic timer and specific frame timer structs and impls


#### Archived Actions
- [x] Switch to immutable buffer
- [X] impiment Mesh and Model API
- [x] recreate swapchain
- [x] Switch to indexed drawing
- [x] switch to using result again
- [x] build a super setting object with all other settings that can then just be added on to 
- [x] Colored Quad Example
- [x] uniform buffers
- [x] fix triangle example

[L1]: https://github.com/amethyst/amethyst/blob/master/amethyst_renderer/src/color.rs