
#version 330 core

in vec2 tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform int pixel_size;

void main() {
    ivec2 tex_size = textureSize(tex, 0);
    ivec2 pixel_coords = ivec2(tex_coords * tex_size / pixel_size);
    color = texture(tex, vec2(pixel_coords) / tex_size);
}
