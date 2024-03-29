use super::*;

#[derive(Clone, Debug)]
pub struct HeatMapPlot {
    pub column_data: Vec<f64>,
    pub row_data: Vec<f64>,
    pub value: Vec<Vec<f64>>,
}

impl Plot for HeatMapPlot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()> {
        // We assume that `row_data` and `column_data` are evenly spaced. If not,
        // additional calculations would be required.
        let row_interval = if self.row_data.len() > 1 {
            self.row_data[1] - self.row_data[0]
        } else {
            1.0
        };

        let column_interval = if self.column_data.len() > 1 {
            self.column_data[1] - self.column_data[0]
        } else {
            1.0
        };

        let mut chart = ChartBuilder::on(drawing_area)
            .caption("HeatMap", ("sans-serif", 40).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                self.column_data[0]..self.column_data.last().unwrap() + column_interval,
                self.row_data[0]..self.row_data.last().unwrap() + row_interval,
            )?;

        let max_value = self
            .value
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NAN, f64::max);

        chart.draw_series(
            self.clone()
                .value
                .iter()
                .enumerate()
                .flat_map(|(y_idx, row)| {
                    row.iter().enumerate().map(move |(x_idx, &value)| {
                        let x = self.column_data[x_idx];
                        let y = self.row_data[y_idx];
                        (x, y, value)
                    })
                })
                .map(|(x, y, value)| {
                    Rectangle::new(
                        [(x, y), (x + column_interval, y + row_interval)],
                        HSLColor(
                            240.0 / 360.0 - 240.0 / 360.0 * (value / max_value),
                            0.7,
                            0.1 + 0.4 * value / max_value,
                        )
                        .filled(),
                    )
                }),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_heatmap_plot() {
        let mut figure = Figure::new("test_add_heatmap_plot", None);
        let heatmap = HeatMapPlot {
            column_data: vec![0.0, 1.0, 2.0],
            row_data: vec![0.0, 1.0],
            value: vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]],
        };
    }

    #[test]
    fn create_heatmap_plot() {
        let mut figure = Figure::new("test_create_heatmap_plot", None);
        let heatmap = HeatMapPlot {
            column_data: vec![0.0, 1.0, 2.0],
            row_data: vec![0.0, 1.0],
            value: vec![vec![2.0, 4.0, 2.0], vec![3.0, 1.0, 5.0]],
        };
        figure.add_plot(heatmap);
        figure.create().unwrap();
        assert!(std::path::Path::new("test_create_heatmap_plot.png").exists());
        std::fs::remove_file("test_create_heatmap_plot.png").unwrap();
    }
}
