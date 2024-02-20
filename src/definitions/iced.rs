use blue_engine::{Camera, EnginePlugin, ObjectStorage, Renderer, Window as Win};
use iced_runtime::{program::State, Command, Program};
use iced_wgpu::{Backend, Renderer as IcedRenderer, Settings};
use iced_winit::{
    core::{Element, Font, Pixels, Size},
    runtime::Debug,
    style::Theme,
};

pub struct IcedProgram {}
impl Program for IcedProgram {
    type Renderer = IcedRenderer;
    type Message = ();
    type Theme = Theme;

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme, Self::Renderer> {
        todo!()
    }
}

/// The iced plugin
pub struct Iced {
    renderer: IcedRenderer,
    debug: Debug,
}

impl Iced {
    /// Creates the iced context and platform details
    pub fn new(
        event_loop: &blue_engine::EventLoop<()>,
        window: &Win,
        blue_renderer: &mut Renderer,
    ) -> Self {
        let physical_size = window.inner_size();
        let mut viewport = iced_graphics::Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
        );

        let mut debug = Debug::new();
        let tex_format = blue_renderer
            .surface
            .as_ref()
            .unwrap()
            .get_capabilities(&blue_renderer.adapter)
            .formats[0];
        let mut iced_renderer = IcedRenderer::new(
            Backend::new(
                &blue_renderer.device,
                &blue_renderer.queue,
                Settings::default(),
                tex_format,
            ),
            Font::default(),
            Pixels(16.0),
        );

        let mut state = State::new(
            IcedProgram {},
            viewport.logical_size(),
            &mut iced_renderer,
            &mut debug,
        );

        Self {
            renderer: iced_renderer,
            debug,
        }
    }

    pub fn ui<F: FnOnce()>(&mut self, callback: F, window: &Win) {}
}

impl EnginePlugin for Iced {
    /// updates the inputs and events
    fn update_events(
        &mut self,
        _renderer: &mut Renderer,
        _window: &Win,
        _objects: &mut ObjectStorage,
        _events: &blue_engine::Event<()>,
        _input: &blue_engine::InputHelper,
        _camera: &mut Camera,
    ) {
        match _events {
            blue_engine::Event::WindowEvent { event, .. } => {
                //? has a return, maybe useful in the future
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        renderer: &mut blue_engine::Renderer,
        window: &blue_engine::Window,
        _objects: &mut ObjectStorage,
        _camera: &mut blue_engine::Camera,
        _input: &blue_engine::InputHelper,
        encoder: &mut blue_engine::CommandEncoder,
        view: &blue_engine::TextureView,
    ) {
    }
}
