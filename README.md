# EffectCraft Studio

An open-source, sovereign motion graphics and visual effects compositing software built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![EffectCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode motion graphics UI with keyframe Bézier curve editors, composition viewer, and render queue.
- **`crates/engine`**: Multi-layer compositing engine, motion blur rasterizer, and procedural animation graph.

## Legal & Compliance Notice

EffectCraft is an independent open-source motion graphics software. It is not affiliated with Adobe Inc. Adobe, After Effects, and Creative Cloud are trademarks of Adobe Inc. Keyframe interpolation and timing graph coordinates are standard animation techniques (17 U.S.C. § 102(b)).

## License

Dual-licensed under MIT OR Apache-2.0.
