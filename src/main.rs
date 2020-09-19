use valora::prelude::*;

fn main() -> Result<()> {
    run_fn(Options::from_args(), |_gpu, world, _rng| {
        Ok(move |ctx: Context, canvas: &mut Canvas| {
            canvas.set_color(LinSrgb::new(1., 1., 1.));
            canvas.paint(Filled(ctx.world));

            let triangle = Ngon::triangle(world.center(), 200.);
            let repeated = std::iter::successors(Some(triangle), |t| {
                Some(t.clone().scale(0.9))
            });

            for triangle in repeated.take(15) {
                canvas.paint(Stroked {
                    width: 2.,
                    element: triangle,
                });
            }

            let max_radius = world.width / 3.;
            let radius = ctx.time.as_secs_f32().cos().abs() * max_radius;

            canvas.set_color(LinSrgb::new(1.,0.,0.));
            canvas.paint(Filled(Ellipse::circle(world.center(), radius)));
        })
    })
}