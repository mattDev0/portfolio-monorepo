use leptos::prelude::*;

pub const SPARKLINE_HEIGHT: f32 = 18.0;
pub const SPARKLINE_WIDTH: f32 = 80.0;
pub const SPARKLINE_PADDING: f32 = 1.0;

pub fn calculate_max_value(data: &[f32], max_override: Option<f32>) -> f32 {
    if let Some(m) = max_override {
        m
    } else {
        let highest = data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        (highest * 1.1).max(10.0)
    }
}

pub fn calculate_sparkline_points(data: &[f32], max_override: Option<f32>) -> Option<String> {
    if data.len() < 2 {
        return None;
    }

    let min_val = 0.0;
    let max_val = calculate_max_value(data, max_override);
    let range = if (max_val - min_val) != 0.0 {
        max_val - min_val
    } else {
        1.0
    };

    let len_minus_one = (data.len() - 1) as f32;
    let width_inner = SPARKLINE_WIDTH - 2.0 * SPARKLINE_PADDING;
    let height_inner = SPARKLINE_HEIGHT - 2.0 * SPARKLINE_PADDING;

    let points = data
        .iter()
        .enumerate()
        .map(|(index, &val)| {
            let x = SPARKLINE_PADDING + (index as f32 / len_minus_one) * width_inner;
            let y = SPARKLINE_PADDING + (1.0 - (val - min_val) / range) * height_inner;
            format!("{},{}", x, y)
        })
        .collect::<Vec<String>>()
        .join(" ");

    Some(points)
}

#[component]
pub fn Sparkline(
    #[prop(into)] data: Signal<Vec<f32>>,
    #[prop(into)] color: &'static str,
    #[prop(optional)] max: Option<f32>,
) -> impl IntoView {
    view! {
        {move || {
            let current_data = data.get();
            match calculate_sparkline_points(&current_data, max) {
                Some(points) => {
                    view! {
                        <svg class="w-[80px] h-[18px] opacity-85" viewBox=format!("0 0 {} {}", SPARKLINE_WIDTH, SPARKLINE_HEIGHT)>
                            <polyline
                                fill="none"
                                style=format!("stroke: {}", color)
                                stroke-width="1.2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                points=points
                            />
                        </svg>
                    }.into_any()
                }
                None => {
                    view! {
                        <div class="h-4 flex items-center justify-center text-[10px] text-[var(--color-text-muted)] font-mono tracking-wider">
                            "syncing..."
                        </div>
                    }.into_any()
                }
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_fewer_than_two_points() {
        assert_eq!(calculate_sparkline_points(&[], None), None);
        assert_eq!(calculate_sparkline_points(&[10.0], None), None);
    }

    #[test]
    fn test_sparkline_known_hand_computed_input() {
        // data = [0.0, 100.0], max = Some(100.0)
        // x0 = 1 + (0/1) * 78 = 1.0, y0 = 1 + (1 - 0/100) * 16 = 17.0
        // x1 = 1 + (1/1) * 78 = 79.0, y1 = 1 + (1 - 100/100) * 16 = 1.0
        let points = calculate_sparkline_points(&[0.0, 100.0], Some(100.0));
        assert_eq!(points, Some("1,17 79,1".to_string()));
    }

    #[test]
    fn test_sparkline_derived_max() {
        // data = [10.0, 50.0, 10.0], no max given.
        // max should be (50.0 * 1.1).max(10.0) = 55.0
        let derived = calculate_max_value(&[10.0, 50.0, 10.0], None);
        assert!((derived - 55.0).abs() < 1e-4);

        // When all values are 0, max should default to 10.0
        let zero_derived = calculate_max_value(&[0.0, 0.0], None);
        assert_eq!(zero_derived, 10.0);

        let points = calculate_sparkline_points(&[10.0, 50.0, 10.0], None).unwrap();
        let parts: Vec<&str> = points.split(' ').collect();
        assert_eq!(parts.len(), 3);
    }
}
