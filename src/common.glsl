    attribute vec3 vPosition;
    attribute vec3 vColor;
    varying vec3 color;
    void main()
    {
        color = vColor + vPosition + vec3(1.,0.,0.);
        gl_Position = vec4(vPosition, 1.0);
    }