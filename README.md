# Hobby
Hobby project for learning how to put together a vulkan renderer in Rust

### Vulkan Debug Info
* Install [Vulkan SDK](https://vulkan.lunarg.com/doc/view/latest/linux/getting_started.html)
* set the VK_INSTANCE_LAYERS environment variable and debug info will be printed
* `export VK_INSTANCE_LAYERS=VK_LAYER_LUNARG_standard_validation`
* `export VK_INSTANCE_LAYERS=VK_LAYER_LUNARG_api_dump`
* Rust Backtrace: `export RUST_BACKTRACE=1`
