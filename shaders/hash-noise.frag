precision mediump float;

// uniforms
uniform vec4 gain;
uniform vec4 bias;
uniform float seed;

// this is the same variable we declared in the vertex shader
// we need to declare it here too!
varying vec2 vTexCoord;


vec2 hash22(vec2 p) {
    float n = sin(dot(p, vec2(41, 289)));
    return fract(vec2(262144, 32768)*n);
}

void main() {
    vec2 vseed = vec2(seed);
    float c = hash22(vseed + vTexCoord).r;
    gl_FragColor = vec4(c, c, c, 1.0);
}