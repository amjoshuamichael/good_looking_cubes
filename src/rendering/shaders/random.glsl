int setup_rand(vec2 pos) {
    return int(pos.x * 5000.0) * 1973 + int(pos.y * 5000.0) * 9277 | 1;
}

int rand(int seed) {
    seed = seed ^ (seed << 13);
    seed = seed ^ (seed >> 17);
    seed = seed ^ (seed << 5);
    return seed;
}

vec3 rand_vector(int seed) {
    int x = rand(seed);
    int y = rand(x);
    int z = rand(y);
    return normalize(vec3(x, y, z));
}

