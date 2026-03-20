//! wgpu + font atlas rendering for [`punk_terminal::TerminalGrid`].

use std::collections::HashMap;
use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use fontdue::Font;
use punk_terminal::TerminalGrid;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

const FONT_PX: f32 = 14.0;
const FALLBACK_CHAR: char = '?';

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuVertex {
    pos: [f32; 2],
    uv: [f32; 2],
    fg: [f32; 3],
    bg: [f32; 3],
}

pub struct TerminalRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    atlas_texture: wgpu::Texture,
    #[allow(dead_code)]
    sampler: wgpu::Sampler,
    cell_w_px: f32,
    cell_h_px: f32,
    baseline_px: f32,
    atlas_glyphs: HashMap<char, AtlasGlyph>,
    fallback_glyph: AtlasGlyph,
}

#[derive(Clone, Copy)]
struct AtlasGlyph {
    uv: [f32; 4],
    width: f32,
    height: f32,
    xmin: f32,
    ymin: f32,
    advance: f32,
}

impl TerminalRenderer {
    pub fn new(window: Arc<Window>) -> Result<Self, String> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .map_err(|e| format!("create_surface: {e}"))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| "no wgpu adapter".to_string())?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .map_err(|e| format!("request_device: {e}"))?;

        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| "no surface config".to_string())?;
        config.format = format;
        surface.configure(&device, &config);

        const FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
        let font = Font::from_bytes(FONT, fontdue::FontSettings::default())
            .map_err(|_| "bad font bytes".to_string())?;
        let line = font
            .horizontal_line_metrics(FONT_PX)
            .ok_or_else(|| "no line metrics".to_string())?;
        let (cell_w_px, cell_h_px) = terminal_cell_pixel_size(&font, FONT_PX, line.new_line_size);
        let baseline_px = line.ascent.ceil();

        let (atlas_tex, atlas_view, glyphs, fallback_glyph, _atlas_size_px) =
            build_atlas(&device, &queue, &font, FONT_PX)?;

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("terminal.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x3, 3 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            bind_group,
            atlas_texture: atlas_tex,
            sampler,
            cell_w_px,
            cell_h_px,
            baseline_px,
            atlas_glyphs: glyphs,
            fallback_glyph,
        })
    }

    pub fn cell_pixel_size(&self) -> (f32, f32) {
        (self.cell_w_px, self.cell_h_px)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Pass the window inner size in physical pixels so cell quads match the grid used for the PTY
    /// (avoids micro‑columns when the swapchain size lags the window).
    pub fn draw_grid(
        &mut self,
        grid: &TerminalGrid,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let w = width.max(1) as f32;
        let h = height.max(1) as f32;
        if width != self.config.width || height != self.config.height {
            self.config.width = width.max(1);
            self.config.height = height.max(1);
            self.surface.configure(&self.device, &self.config);
        }

        let cols = grid.cols() as usize;
        let rows = grid.rows() as usize;
        let cw = self.cell_w_px.max(1.0).round();
        let ch = self.cell_h_px.max(1.0).round();
        let content_w = cw * cols as f32;
        let content_h = ch * rows as f32;
        let origin_x = ((w - content_w) * 0.5).max(0.0).floor();
        let origin_y = ((h - content_h) * 0.5).max(0.0).floor();

        let mut verts: Vec<GpuVertex> = Vec::with_capacity(cols * rows * 6);
        let mut text_verts: Vec<GpuVertex> = Vec::with_capacity(cols * rows * 6);
        let cells = grid.cells();

        for row in 0..rows {
            for col in 0..cols {
                let cell = &cells[row * cols + col];
                let c = cell.ch;
                let glyph = self
                    .atlas_glyphs
                    .get(&c)
                    .copied()
                    .unwrap_or(self.fallback_glyph);
                let fg = rgb01(&cell.fg);
                let bg = rgb01(&cell.bg);
                let x0 = origin_x + col as f32 * cw;
                let y0 = origin_y + row as f32 * ch;
                let x1 = x0 + cw;
                let y1 = y0 + ch;
                let ndc = |px: f32, py: f32| [px / w * 2.0 - 1.0, 1.0 - py / h * 2.0];
                let p00 = ndc(x0, y0);
                let p10 = ndc(x1, y0);
                let p01 = ndc(x0, y1);
                let p11 = ndc(x1, y1);
                // Background pass: fill cell exactly.
                quad(&mut verts, p00, p10, p01, p11, glyph.uv, bg, bg);

                if c == ' ' || glyph.width <= 0.0 || glyph.height <= 0.0 {
                    continue;
                }
                let glyph_x = (x0 + ((cw - glyph.advance).max(0.0) * 0.5) + glyph.xmin).round();
                // fontdue::Metrics::ymin is the distance from baseline to glyph bottom.
                // Convert to top-left in pixel space: baseline - (height + ymin).
                let glyph_top = (y0 + self.baseline_px - glyph.height - glyph.ymin).round();
                let gx0 = glyph_x.max(x0);
                let gy0 = glyph_top.max(y0);
                let gx1 = (glyph_x + glyph.width).min(x1);
                let gy1 = (glyph_top + glyph.height).min(y1);
                if gx1 <= gx0 || gy1 <= gy0 {
                    continue;
                }
                let uv = clip_uv_to_quad(
                    glyph.uv,
                    glyph_x,
                    glyph_top,
                    glyph.width,
                    glyph.height,
                    gx0,
                    gy0,
                    gx1,
                    gy1,
                );
                let gp00 = ndc(gx0, gy0);
                let gp10 = ndc(gx1, gy0);
                let gp01 = ndc(gx0, gy1);
                let gp11 = ndc(gx1, gy1);
                quad(&mut text_verts, gp00, gp10, gp01, gp11, uv, fg, bg);
            }
        }
        verts.extend(text_verts);

        let vb = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                self.surface
                    .get_current_texture()
                    .map_err(|e| format!("surface texture: {e}"))?
            }
            Err(e) => return Err(format!("surface texture: {e}")),
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw(0..verts.len() as u32, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}

/// One terminal cell in pixels: at least the glyph box and advance, so column count stays sane.
fn terminal_cell_pixel_size(font: &Font, px: f32, new_line: f32) -> (f32, f32) {
    let (mm, _) = font.rasterize('M', px);
    let m = font.metrics('M', px);
    let adv = m.advance_width;
    if !adv.is_finite() || adv <= 0.0 {
        return (8.0_f32, new_line.max(12.0).max(1.0));
    }
    let w_from_glyph = mm.width as f32 + 2.0;
    let cell_w = adv.max(w_from_glyph).ceil().clamp(6.0, 256.0);
    let h_from_glyph = mm.height as f32 + 2.0;
    let cell_h = new_line.max(h_from_glyph).ceil().clamp(6.0, 256.0);
    (cell_w, cell_h)
}

fn rgb01(c: &[u8; 3]) -> [f32; 3] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    ]
}

fn quad(
    verts: &mut Vec<GpuVertex>,
    p00: [f32; 2],
    p10: [f32; 2],
    p01: [f32; 2],
    p11: [f32; 2],
    uv: [f32; 4],
    fg: [f32; 3],
    bg: [f32; 3],
) {
    let u0 = uv[0];
    let v0 = uv[1];
    let u1 = uv[2];
    let v1 = uv[3];
    verts.push(GpuVertex {
        pos: p00,
        uv: [u0, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p10,
        uv: [u1, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p01,
        uv: [u0, v1],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p10,
        uv: [u1, v0],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p11,
        uv: [u1, v1],
        fg,
        bg,
    });
    verts.push(GpuVertex {
        pos: p01,
        uv: [u0, v1],
        fg,
        bg,
    });
}

fn clip_uv_to_quad(
    uv: [f32; 4],
    glyph_x: f32,
    glyph_y: f32,
    glyph_w: f32,
    glyph_h: f32,
    qx0: f32,
    qy0: f32,
    qx1: f32,
    qy1: f32,
) -> [f32; 4] {
    let [u0, v0, u1, v1] = uv;
    let du = (u1 - u0).max(0.0);
    let dv = (v1 - v0).max(0.0);
    let tx0 = ((qx0 - glyph_x) / glyph_w).clamp(0.0, 1.0);
    let tx1 = ((qx1 - glyph_x) / glyph_w).clamp(0.0, 1.0);
    let ty0 = ((qy0 - glyph_y) / glyph_h).clamp(0.0, 1.0);
    let ty1 = ((qy1 - glyph_y) / glyph_h).clamp(0.0, 1.0);
    [u0 + du * tx0, v0 + dv * ty0, u0 + du * tx1, v0 + dv * ty1]
}

fn build_atlas(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    font: &Font,
    px: f32,
) -> Result<
    (
        wgpu::Texture,
        wgpu::TextureView,
        HashMap<char, AtlasGlyph>,
        AtlasGlyph,
        (f32, f32),
    ),
    String,
> {
    const ATLAS_W: usize = 2048;
    const ATLAS_H: usize = 1024;
    let mut pixels = vec![0u8; ATLAS_W * ATLAS_H];
    let mut glyphs = HashMap::new();
    let mut x: usize = 2;
    let mut y: usize = 2;
    let mut row_h: usize = 0;

    for code in 32u8..=126u8 {
        let ch = code as char;
        let (metrics, bitmap) = font.rasterize(ch, px);
        let bw = metrics.width;
        let bh = metrics.height;
        let (gw, gh, has_bmp) = if bw > 0 && bh > 0 {
            (bw, bh, true)
        } else {
            let m = font.metrics(ch, px);
            (
                m.advance_width.ceil().max(1.0) as usize,
                m.height.max(1),
                false,
            )
        };
        if x + gw + 2 > ATLAS_W {
            x = 2;
            y += row_h + 2;
            row_h = 0;
        }
        if y + gh + 2 > ATLAS_H {
            return Err("font atlas full".to_string());
        }
        row_h = row_h.max(gh);
        if has_bmp {
            for by in 0..bh {
                for bx in 0..bw {
                    let a = bitmap[by * bw + bx];
                    pixels[(y + by) * ATLAS_W + x + bx] = a;
                }
            }
        }
        let m = font.metrics(ch, px);
        let inset_u = 0.5 / ATLAS_W as f32;
        let inset_v = 0.5 / ATLAS_H as f32;
        let u0 = x as f32 / ATLAS_W as f32 + inset_u;
        let v0 = y as f32 / ATLAS_H as f32 + inset_v;
        let u1 = (x + gw) as f32 / ATLAS_W as f32 - inset_u;
        let v1 = (y + gh) as f32 / ATLAS_H as f32 - inset_v;
        glyphs.insert(
            ch,
            AtlasGlyph {
                uv: [u0, v0, u1.max(u0), v1.max(v0)],
                width: m.width as f32,
                height: m.height as f32,
                xmin: m.xmin as f32,
                ymin: m.ymin as f32,
                advance: m.advance_width.max(1.0),
            },
        );
        x += gw + 2;
    }

    let fallback_glyph = glyphs
        .get(&FALLBACK_CHAR)
        .copied()
        .or_else(|| glyphs.get(&' ').copied())
        .unwrap_or(AtlasGlyph {
            uv: [0.0, 0.0, 0.001, 0.001],
            width: 0.0,
            height: 0.0,
            xmin: 0.0,
            ymin: 0.0,
            advance: 1.0,
        });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: ATLAS_W as u32,
            height: ATLAS_H as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &pixels,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(ATLAS_W as u32),
            rows_per_image: Some(ATLAS_H as u32),
        },
        wgpu::Extent3d {
            width: ATLAS_W as u32,
            height: ATLAS_H as u32,
            depth_or_array_layers: 1,
        },
    );
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    Ok((
        texture,
        view,
        glyphs,
        fallback_glyph,
        (ATLAS_W as f32, ATLAS_H as f32),
    ))
}
