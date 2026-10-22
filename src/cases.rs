use raylib::prelude::*;
use crate::triangle::Triangle;

pub struct TestCase {
    pub name: &'static str,
    pub triangles: Vec<Triangle>,
}

pub fn big_triangle() -> Triangle {
    Triangle::new(
        Vector2::new(150.0, 480.0),
        Vector2::new(420.0, 120.0),
        Vector2::new(700.0, 430.0),
        Color::new(0, 200, 100, 255),
    )
}

pub fn test_cases() -> Vec<TestCase> {
    let big = big_triangle();

    vec![
        TestCase {
            name: "pequeno_y_grande",
            triangles: vec![
                Triangle::new(
                    Vector2::new(40.0, 40.0),
                    Vector2::new(60.0, 44.0),
                    Vector2::new(48.0, 62.0),
                    Color::new(230, 210, 60, 255),
                ),
                big,
            ],
        },
        TestCase {
            name: "horizontal_y_vertical",
            triangles: vec![
                Triangle::new(
                    Vector2::new(100.0, 100.0),
                    Vector2::new(350.0, 100.0),
                    Vector2::new(200.0, 300.0),
                    Color::new(50, 100, 220, 255),
                ),
                Triangle::new(
                    Vector2::new(450.0, 100.0),
                    Vector2::new(450.0, 400.0),
                    Vector2::new(700.0, 350.0),
                    Color::new(220, 60, 60, 255),
                ),
            ],
        },
        TestCase {
            name: "orden_inverso",
            triangles: vec![Triangle::new(big.c, big.b, big.a, big.color)],
        },
        TestCase {
            name: "parcialmente_fuera",
            triangles: vec![Triangle::new(
                Vector2::new(-100.0, 300.0),
                Vector2::new(300.0, -100.0),
                Vector2::new(450.0, 500.0),
                Color::new(200, 100, 220, 255),
            )],
        },
        TestCase {
            name: "colineal",
            triangles: vec![Triangle::new(
                Vector2::new(100.0, 100.0),
                Vector2::new(300.0, 300.0),
                Vector2::new(500.0, 500.0),
                Color::new(255, 255, 255, 255),
            )],
        },
    ]
}
