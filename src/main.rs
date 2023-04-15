#![allow(dead_code)]
mod objects;
mod ray;
mod vec3;
mod camera;
mod scene;


use scene::Scene;

fn main() {
    let scene = Scene::test_scene();
    scene.render_to_png("test.png").unwrap();
}
