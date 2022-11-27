use plotters::prelude::*;
use ruint::Uint;

fn clamp(x: f64, max: f64) -> f64 {
  if x < 0.0 {
    std::f64::MIN_POSITIVE
  } else if x > max {
    max
  } else {
    x
  }
}

fn accuracy_simplified(m: Uint<1024, 16>, n: f64) -> f64 {
  let power = (4_f64).powf(n);
  let lnm = m.approx_log(std::f64::consts::E);
  1.0 - (lnm / power)
}

fn accuracy(m: Uint<1024, 16>, n: f64) -> f64 {
  let lnm = m.approx_log(std::f64::consts::E);
  let miller_rabin_chance = (0.25_f64).powf(n);
  1.0 - ((miller_rabin_chance * (lnm - 1.0)) / (1.0 + (miller_rabin_chance * (lnm - 1.0))))
}

const OUT_FILE_NAME: &str = "out/accuracy.svg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Config variables
  let max_iterations = 10;
  let max_base = 150;
  let iteration_resolution = 10;
  let base_resolution = 1;

  // Create the drawing area
  let area = SVGBackend::new(OUT_FILE_NAME, (600, 600)).into_drawing_area();
  area.fill(&WHITE)?;

  // Build the chart
  let mut chart = ChartBuilder::on(&area)
    .caption("Miller-Rabin accuracy", ("cambria", 100))
    .build_cartesian_3d::<std::ops::Range<f64>, std::ops::Range<f64>, std::ops::Range<f64>>(
      0.0_f64..max_base.into(),
      0.0_f64..1.0_f64,
      1.0_f64..max_iterations.into(),
    )
    .unwrap();

  // Set up the camera/perspective
  chart.with_projection(|mut pb| {
    pb.yaw = 0.5;
    pb.pitch = 0.3;
    pb.scale = 0.7;
    pb.into_matrix()
  });

  // Draw the error to the graph
  chart.draw_series(
    SurfaceSeries::xoz(
      (-1..(max_base * base_resolution)).map(|f| f as f64 / base_resolution as f64 + 1.0),
      ((1_u32 * iteration_resolution)..(max_iterations * iteration_resolution))
        .map(|f| f as f64 / iteration_resolution as f64),
      |a, b| {
        clamp(
          (accuracy(Uint::from(10).pow(a as usize), b))
            - (accuracy_simplified(Uint::from(10).pow(a as usize), b)),
          1.0,
        )
      },
    )
    .style_func(&|&v| {
      (&HSLColor(
        (1.0 + clamp(60.0 * v * (1.0 / 1.0), 60.0) / 360.0) % 1.0,
        1.0,
        0.6,
      ))
        .into()
    }),
  )?;

  chart.configure_series_labels().draw()?;
  chart
    .configure_axes()
    .x_formatter(&|x| format!("10^{}", x.floor() as i32))
    .z_formatter(&|z| format!("{}", z.floor() as i32))
    .draw()?;

  area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
  println!("Result has been saved to {}", OUT_FILE_NAME);
  Ok(())
}
/*


  // Draw the graph
  chart
    .draw_series(
      SurfaceSeries::xoz(
        (-1..(max_base * base_resolution)).map(|f| f as f64 / base_resolution as f64 + 1.0),
        ((3_u32 * iteration_resolution)..(max_iterations * iteration_resolution))
          .map(|f| f as f64 / iteration_resolution as f64),
        |a, b| accuracy_simplified(Uint::from(10).pow(a as usize), b),
      )
      .style_func(&|&y| (&HSLColor((21.0 - (20.0 * y)) / 360.0, 1.0, 0.7 - (0.3 * y))).into()),
    )?
    .label("simplified")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.mix(0.5).filled()));
  println!("Simplified chart done");

  chart
    .draw_series(
      SurfaceSeries::xoz(
        (-1..(max_base * base_resolution)).map(|f| f as f64 / base_resolution as f64 + 1.0),
        ((3_u32 * iteration_resolution)..(max_iterations * iteration_resolution))
          .map(|f| f as f64 / iteration_resolution as f64),
        |a, b| accuracy(Uint::from(10).pow(a as usize), b),
      )
      .style_func(&|&y| (&HSLColor((221.0 - (20.0 * y)) / 360.0, 1.0, 0.7 - (0.3 * y))).into()),
    )?
    .label("accurate")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], PURPLE.mix(0.5).filled()));
  println!("Accurate chart done");

*/
