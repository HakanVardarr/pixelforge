# PixelForge

PixelForge is a software renderer written in Rust, utilizing the `minifb` library for window management and pixel drawing. This project aims to explore rendering techniques from scratch, building everything manually to gain a deeper understanding of computer graphics.

## Features
- Render 2D and 3D shapes manually
- Support for basic shading techniques
- Exploration of custom algorithms for transformations and rasterization
- Educational focus on software rendering fundamentals

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- Basic knowledge of Rust programming and graphics concepts

## Roadmap

The roadmap outlines the development milestones for PixelForge. This project focuses on building a software renderer step by step, progressing from fundamental techniques to advanced features.

### Phase 1: Setup and Basics
- [x] Set up the Rust project with `minifb` for window and framebuffer management.
- [x] Implement basic pixel plotting to display individual pixels on the screen.
- [x] Add functionality to clear and refresh the screen.

### Phase 2: 2D Rendering
- [ ] Implement line drawing using Bresenham's algorithm.
- [ ] Add triangle rasterization for filling 2D shapes.
- [ ] Develop color interpolation for smooth gradients within 2D primitives.

### Phase 3: 3D Rendering
- [ ] Implement a basic 3D pipeline (vertices, edges, and faces).
- [ ] Add a perspective projection matrix for 3D to 2D transformation.
- [ ] Implement backface culling to optimize rendering.

### Phase 4: Shading and Lighting
- [ ] Introduce flat shading for uniform color across faces.
- [ ] Add Gouraud shading for vertex-based color interpolation.
- [ ] Experiment with Phong shading for smoother lighting effects.

### Phase 5: Advanced Features
- [ ] Support texture mapping for adding detail to 3D models.
- [ ] Add basic camera movement and controls.
- [ ] Explore anti-aliasing techniques to smooth edges.

### Phase 6: Optimization and Polishing
- [ ] Optimize rendering pipeline for performance.
- [ ] Add configuration options for resolution and framerate.
- [ ] Write comprehensive documentation and examples.

---

This roadmap is a flexible guide and may evolve as the project progresses.

## Acknowledgments

- Special thanks to the creators of the [`minifb`](https://github.com/emoon/rust-minifb) library for making a lightweight and straightforward framebuffer library.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions, feedback, or collaboration opportunities, feel free to reach out:

- **Email**: [hakovardar@example.com](mailto:hakovardar@gmail.com)
- **GitHub**: [https://github.com/HakanVardarr](https://github.com/HakanVardarr)




