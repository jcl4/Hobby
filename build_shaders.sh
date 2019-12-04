rm resources/shaders/solid_color.vert.spv
glslc resources/shaders/solid_color.vert -o resources/shaders/solid_color.vert.spv

rm resources/shaders/solid_color.frag.spv
glslc resources/shaders/solid_color.vert -o resources/shaders/solid_color.frag.spv