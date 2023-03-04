use std::collections::HashMap;

use crate::{traits::ToIupac, uni::IupacNucleotide};

use super::Export;

#[derive(Debug)]
pub enum SvgRenderMode {
    Circular,
    Linear,
}

#[derive(Debug)]
pub struct SvgExportConfig {
    pub render_mode: SvgRenderMode,
}

impl SvgExportConfig {
    pub fn new(render_mode: SvgRenderMode) -> Self {
        Self { render_mode }
    }

    pub fn circular() -> Self {
        Self {
            render_mode: SvgRenderMode::Circular,
        }
    }

    pub fn linear() -> Self {
        Self {
            render_mode: SvgRenderMode::Linear,
        }
    }
}

pub struct SvgExport {
    pub config: SvgExportConfig,
    pub sequence: Vec<IupacNucleotide>,
}

impl SvgExport {
    pub fn new<T>(config: SvgExportConfig, sequence: &[T]) -> Self
    where
        T: ToIupac,
    {
        SvgExport {
            config,
            sequence: sequence.iter().map(|n| n.to_iupac()).collect(),
        }
    }

    fn color_table() -> HashMap<IupacNucleotide, &'static str> {
        use IupacNucleotide::*;
        HashMap::from([
            (A, "hsla(241, 50%, 50%, 1)"),
            (T, "hsla(51, 50%, 50%, 1)"),
            (G, "hsla(124, 50%, 50%, 1)"),
            (C, "hsla(0, 50%, 50%, 1)"),
        ])
    }

    fn export_circular_sequence(&self) -> String {
        let colors = Self::color_table();
        let default_color = "hsla(241, 0%, 50%, 1)";

        // Basic parameters
        let r = 250_f32; // radius
        let w = r * 2_f32; // width
        let h = r * 2_f32; // height
        let cx = w / 2_f32; // center x
        let cy = h / 2_f32; // center y

        // Step size per nucleotide in degrees
        let step = 360_f32 / self.sequence.len() as f32;

        // Find consecutive nucleotides
        let consecutive_nucleotides = {
            let mut buf = Vec::<(IupacNucleotide, usize)>::new();
            let mut last_nuc: Option<IupacNucleotide> = None;
            let mut curr_chain_size = 1_usize;
            for n in self.sequence.iter() {
                let is_same = last_nuc == Some(*n);
                let new_chain_size = if is_same { curr_chain_size + 1 } else { 1 };
                if !is_same && last_nuc.is_some() {
                    buf.push((last_nuc.unwrap() /* safe */, curr_chain_size));
                }
                curr_chain_size = new_chain_size;
                last_nuc = Some(*n);
            }
            if let Some(n) = last_nuc {
                buf.push((n, curr_chain_size));
            }
            buf
        };

        // Helper function to generate SVG path parameters for an arc
        let deg_to_rad = std::f32::consts::PI / 180_f32;
        let arc = |start_deg: f32, end_deg: f32, stroke_width: f32| {
            let r = r - stroke_width;
            let start_rad = deg_to_rad * start_deg;
            let end_rad = deg_to_rad * (end_deg + 0.01);
            let x1 = start_rad.cos() * r + cx;
            let y1 = start_rad.sin() * r + cy;
            let x2 = end_rad.cos() * r + cx;
            let y2 = end_rad.sin() * r + cy;
            format!("M {} {} A {} {} 0 0 1 {} {}", x1, y1, r, r, x2, y2)
        };

        // Generate segments from consecutive nucleotides
        let segments = {
            let mut last_deg = 0_f32;
            let mut buf = Vec::<String>::new();
            for (n, len) in consecutive_nucleotides {
                let color = colors.get(&n).unwrap_or(&default_color);
                let stroke_width = 20_f32;
                let step = step * len as f32;
                let arc = arc(last_deg, last_deg + step as f32, stroke_width);
                buf.push(format!(
                    r###"<path d="{d}" fill="none" stroke="{color}" stroke-width="{stroke_width}" />"###,
                    d = arc, color = color, stroke_width = stroke_width,
                ));
                last_deg += step;
            }
            buf
        };

        // Assemble final svg
        format!(
            r###"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}">{segments}</svg>"###,
            width = w,
            height = h,
            segments = segments.join("")
        )
    }

    fn export_linear_sequence(&self) -> String {
        todo!()
    }
}

impl Export for SvgExport {
    type Output = String;

    fn export(&self) -> String {
        match self.config.render_mode {
            SvgRenderMode::Circular => self.export_circular_sequence(),
            SvgRenderMode::Linear => self.export_linear_sequence(),
        }
    }
}
