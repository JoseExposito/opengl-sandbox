use nalgebra_glm as glm;

fn main() {
    // Translate the vector (1,0,0) by (1,1,0) = (2,1,0)
    let vec = glm::vec4(1.0, 0.0, 0.0, 1.0);
    let trans_vec = glm::vec3(1.0, 1.0, 0.0);

    let trans = glm::translate(&glm::Mat4::identity(), &trans_vec);
    let translated_vec = trans * vec;

    println!(
        "Translated vector: ({},{},{})",
        translated_vec.x, translated_vec.y, translated_vec.z
    );
}
