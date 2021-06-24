#type vertex
#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 col;

uniform mat4 uTransform;

out struct {
	vec2 uv;
	vec4 color;
} vs;

void main() {
  vs.color = col;
  vs.uv = uv;
  gl_Position = uTransform * vec4(pos, 0.0, 1.0);
}

#type fragment
#version 330 core

in struct {
	vec2 uv;
	vec4 color;
} vs;

out vec4 color;

uniform sampler2D uFontTexture;

void main() {
	color = vs.color * texture(uFontTexture, vs.uv);
}
