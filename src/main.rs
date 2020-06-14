use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    use futures::executor::block_on;

    let mut state = block_on(State::new(&window));


    event_loop.run(move |event, _, control_flow| {
	    match event {
		Event::WindowEvent {
		    ref event,
		    window_id,
		} if window_id == window.id() => if !state.input(event) {
		    match event {
		        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
		        WindowEvent::KeyboardInput {
		            input,
		            ..
		        } => {
		            match input {
		                KeyboardInput {
		                    state: ElementState::Pressed,
		                    virtual_keycode: Some(VirtualKeyCode::Escape),
		                    ..
		                } => *control_flow = ControlFlow::Exit,
		                _ => {}
		            }
		        }
		        WindowEvent::Resized(physical_size) => {
		            state.resize(*physical_size);
		        }
		        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
		            state.resize(**new_inner_size);
		        }
		        _ => {}
		    }
		}
		Event::RedrawRequested(_) => {
		    state.update();
		    state.render();
		}
    Event::MainEventsCleared => {
		    window.request_redraw();
		}
		_ => {}
	    }
    });
}

struct State {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

		clear_color: wgpu::Color,
		
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await.unwrap();
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;
        
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

				let clear_color = wgpu::Color::BLACK;
				
        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,

						clear_color,
						
            size,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    	self.sc_desc.width = new_size.width;
    	self.sc_desc.height = new_size.height;
    	self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn update(&mut self) {

    }

    fn render(&mut self) {
       let frame = self.swap_chain.get_next_texture()
       	.expect("Timeout getting texture");
       let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
       	label: Some("Render Encoder"),
   	});
   	{
		let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		    color_attachments: &[
		        wgpu::RenderPassColorAttachmentDescriptor {
		            attachment: &frame.view,
		            resolve_target: None,
		            load_op: wgpu::LoadOp::Clear,
		            store_op: wgpu::StoreOp::Store,
		            clear_color: self.clear_color,
		        }
		    ],
		    depth_stencil_attachment: None,
		});
	}

    	self.queue.submit(&[
    	    encoder.finish()
    	]);
    }
		fn input(&mut self, event: &WindowEvent) -> bool {
				match event {
						WindowEvent::CursorMoved {
								position,
								..
						} => {
								self.clear_color = wgpu::Color {
										r: position.x as f64 / self.size.width as f64,
										g: position.y as f64 / self.size.height as f64,
										b: 0.0,
										a: 1.0,
								};
								true
						}
						_ => false,
				}
		}
}
