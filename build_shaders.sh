rm resources/shaders/solid_color.vert.spv
glslangValidator -V -o resources/shaders/solid_color.vert.spv resources/shaders/solid_color.vert

rm resources/shaders/solid_color.frag.spv
glslangValidator -V -o resources/shaders/solid_color.frag.spv resources/shaders/solid_color.vert