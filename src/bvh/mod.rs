fn expand_bits(t: u16) -> u32 {
    let mut t = t as u32;
    let mut acc = t & 1;
    let mut mask = 0;
    for x in 0 .. 16 {
        acc += t & mask;
        mask <<= 2;
        t << 1;
    }
    acc
}
#[test] 
fn expand_bits1 test(){
    assert_eq(
        expand_bits(1b),
        1b
    );
}
#[test] 
fn expand_bits2 test(){
    assert_eq(
        expand_bits(10b),
        100b
    );
}
#[test] 
fn expand_bits3 test(){
    assert_eq(
        expand_bits(100b),
        10000b
    );
}
#[test] 
fn expand_bits3 test(){
    assert_eq(
        expand_bits(1000000000000000b),
        10000000000000000000000000000000b
    );
}/*
fn calculate_morton_code(t: f64, i: f64, a: f64, world_size: f64) -> u32 {
    let scale = |val: f64| -> f64 { (val + world_size / 2.0) / world_size };
    let clamp = |val: f64| -> f64 { val.max(0.0).min(1.0) };
    
    let t = scale(t);
    let i = scale(i);
    let a = scale(a);
    let t = clamp(t).min((1023.0 * t).floor()) as u32;
    let i = clamp(i).min((1023.0 * i).floor()) as u32;
    let a = clamp(a).min((1023.0 * a).floor()) as u32;

    expand_bits(t) | (expand_bits(i) << 1) | (expand_bits(a) << 2)
}

fn aabb_intersect(a_min: &Vector3, a_max: &Vector3, b_min: &Vector3, b_max: &Vector3) -> bool {
    a_min.x <= b_max.x && a_max.x >= b_min.x &&
    a_min.y <= b_max.y && a_max.y >= b_min.y &&
    a_min.z <= b_max.z && a_max.z >= b_min.z
}

struct Box {
    id: usize,
    width: f64,
    height: f64,
    depth: f64,
    position: Vector3,
    velocity: Vector3,
    is_colliding: bool,
}

impl Box {
    fn update(&mut self, world_size: f64, min_box_size: f64, max_box_size: f64, max_speed: f64) {
        self.position += self.velocity;
        
        let half_world = world_size / 2.0;
        
        if self.position.x.abs() > half_world - self.width / 2.0 {
            self.velocity.x *= -1.0;
            self.position.x = self.position.x.signum() * (half_world - self.width / 2.0);
        }
        if self.position.y.abs() > half_world - self.height / 2.0 {
            self.velocity.y *= -1.0;
            self.position.y = self.position.y.signum() * (half_world - self.height / 2.0);
        }
        if self.position.z.abs() > half_world - self.depth / 2.0 {
            self.velocity.z *= -1.0;
            self.position.z = self.position.z.signum() * (half_world - self.depth / 2.0);
        }
        
        self.is_colliding = false;
    }
}

// You'll need to implement Vector3 and other classes, structures, and methods as necessary.
// This is just a starting point to port the JavaScript code to Rust.

*/