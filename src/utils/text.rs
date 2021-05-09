use crate::definitions::Renderer;
use futures::task::SpawnExt;
use wgpu::util::DeviceExt;
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Section};

#[derive(Debug, Clone, Copy)]
struct TextData {
    content: &'static str,
    position: (f32, f32),
    color: &'static [f32; 4],
    scale: f32,
}
pub struct Text {
    brush: GlyphBrush<()>,
    font: FontArc,
    texts: Vec<TextData>,
    bounds: winit::dpi::PhysicalSize<u32>,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: futures::executor::LocalSpawner,
}

impl Text {
    pub fn new(
        renderer: &Renderer,
        font: Vec<u8>,
        bounds: winit::dpi::PhysicalSize<u32>,
    ) -> anyhow::Result<Self> {
        let font = FontArc::try_from_vec(font)?;
        let brush = GlyphBrushBuilder::using_font(font.clone())
            .build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);
            
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        Ok(Self {
            brush,
            font,
            texts: Vec::new(),
            bounds,
            staging_belt,
            local_pool,
            local_spawner,
        })
    }

    pub fn new_font(&mut self, renderer: &Renderer, font: Vec<u8>) -> anyhow::Result<()> {
        self.font = FontArc::try_from_vec(font)?;
        self.brush = GlyphBrushBuilder::using_font(self.font.clone())
            .build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);

        Ok(())
    }

    pub fn add_text(
        &mut self,
        content: &'static str,
        position: (f32, f32),
        color: &'static [f32; 4],
        scale: f32,
    ) -> anyhow::Result<()> {
        self.texts.push(TextData {
            content,
            position,
            color,
            scale,
        });
        Ok(())
    }

    pub fn compile(&mut self) -> anyhow::Result<()> {
        for i in self.texts.clone() {
            self.brush.queue(Section {
                bounds: (self.bounds.width as f32, self.bounds.height as f32),
                screen_position: i.position,
                text: vec![wgpu_glyph::Text::new(i.content)
                    .with_color(*i.color)
                    .with_scale(i.scale)],
                ..Section::default()
            });
        }

        Ok(())
    }

    pub fn render(&mut self, renderer: &Renderer) -> anyhow::Result<()> {
        let frame = renderer.swap_chain.get_current_frame()?.output;
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("font redraw"),
            });

        self.brush
            .draw_queued(
                &renderer.device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.view,
                self.bounds.width,
                self.bounds.height,
            )
            .expect("Draw Queued");

        self.staging_belt.finish();
        renderer.queue.submit(Some(encoder.finish()));
        self.local_spawner.spawn(self.staging_belt.recall())?;
        self.local_pool.run_until_stalled();

        Ok(())
    }
}
