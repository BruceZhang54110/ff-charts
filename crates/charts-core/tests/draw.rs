#[cfg(test)]
mod tests {
    use plotters::prelude::*;

    #[test]
    fn draw_line() -> Result<(), Box<dyn std::error::Error>> {
        // 1. 定义后端：这里使用 BitMapBackend 将图表保存为 PNG 文件
        let root = BitMapBackend::new("line_chart.png", (1000, 700)).into_drawing_area();
        // 2. 填充背景颜色
        root.fill(&WHITE)?;
        // 3. 创建图表上下文
        let mut chart = ChartBuilder::on(&root)
            .caption("Simple Line Chart", ("sans-serif", 30).into_font())
            // Set the size of the label region
            .margin(50)
            .x_label_area_size(5)
            .y_label_area_size(5)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(0f32..7f32, 0f32..100f32)?; // 定义 x 和 y 轴的范围

        
        // 4. 绘制网格
        let original_style = ShapeStyle {
            color: WHITE.mix(1.0),
            filled: false,
            stroke_width: 1,
        };
        
        // 绘制网格
        chart.configure_mesh()
            // x 轴描述
            .x_desc("")
            .y_desc("")
            // Set how many labels for the X axis at most
            .x_labels(14)
            // Set how many label for the Y axis at most
            //.y_labels(10)
            .y_desc("温度")
            // 禁用x轴网格线
            //.disable_x_mesh()
            .light_line_style(original_style)
            .draw()?;

        chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

        // 5. 绘制数据序列（一条直线）
        
        chart.draw_series(LineSeries::new(
            vec![(0.0, 0.0), (1.0, 10.0), (2.0, 10.0), (3.0, 50.0)], // 数据点
            &RED,                         // 线条颜色
        )).unwrap();

        println!("Generated line_chart.png");
        Ok(())
    }

    
}