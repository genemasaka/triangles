// Define a struct named "Triangle"
// with two fields: "height" and "base"
struct Triangle {
	// A field named "height" of type "u32"
	height: u32,
	// A field named "base" of type "u32"
	base: u32,
}

fn main() {
	// Create an instance of the Triangle struct
	let triangle1 = Triangle {
		// Set the "height" field to 27
		height: 27,
		// Set the "base" field to 64
		base: 64,
	};

	// Calculate the area of the triangle
	let new_area = area(&triangle1);

	// Print the result of the area calculation
	println!("The triangle is {} square pixels", new_area);
}

// A function named "area" that calculates
// the area of a triangle
fn area(triangle: &Triangle) -> f32 {
	// Calculate the area using 0.5 * base * height
	0.5 * triangle.base as f32 * triangle.height as f32
}
