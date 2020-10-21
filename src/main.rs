use valora::prelude::*;

fn main() -> Result<()> {
    run_fn(Options::from_args(), |_gpu, world, _rng| {
        Ok(move |ctx: Context, canvas: &mut Canvas| {
            let fbm = Fbm::new().set_seed(world.seed as u32);
            let shift_by_timed_noise = |(v, strength): (P2, f32)| {
                let sample_point =
                    P3::new(v.x / 100., v.y / 100., ctx.time.as_secs_f32().sin() / 2.);
                let t = fbm.noise(sample_point) * strength;
                v.translate(V2::new(t, t))
            };

            let square = Polygon::from(Ngon::triangle(world.center(), 150.));
            let initial_strength = 15.;
            let splotches =
                std::iter::successors(Some((square, initial_strength)), |(shape, strength)| {
                    let next_strength = strength / 0.5;
                    let next_shape = Polygon::from(
                        shape
                            .clone()
                            .subdivide()
                            .vertices()
                            .map(|v| (v, *strength))
                            .map(shift_by_timed_noise),
                    );

                    Some((next_shape, next_strength))
                })
                .map(|(s, _)| s);

            for splotch in splotches.skip(4).take(2) {
                canvas.paint(Filled(splotch));
            }
        })
    })
}
