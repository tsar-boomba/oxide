mod backend;

use std::{
    collections::HashMap, os::unix::io::FromRawFd, os::unix::io::OwnedFd, path::PathBuf, sync::Arc,
    time::Duration,
};

use framebuffer::Framebuffer;
use smithay::{
    backend::renderer::{
        element::surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
        gles::GlesRenderer,
        utils::{draw_render_elements, on_commit_buffer_handler},
        Frame, Renderer,
    },
    delegate_compositor, delegate_data_device, delegate_seat, delegate_shm, delegate_xdg_shell,
    input::{keyboard::FilterResult, Seat, SeatHandler, SeatState},
    reexports::{
        wayland_protocols,
        wayland_server::{self, protocol::wl_seat, Display},
    },
    utils::{DeviceFd, Rectangle, Serial, Size, Transform},
    wayland::{
        buffer::BufferHandler,
        compositor::{
            with_surface_tree_downward, CompositorClientState, CompositorHandler, CompositorState,
            SurfaceAttributes, TraversalAction,
        },
        data_device::{
            ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
        },
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmState},
    },
};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason},
    protocol::{
        wl_buffer,
        wl_surface::{self, WlSurface},
    },
    Client, ListeningSocket,
};

use crate::backend::Backend;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

impl BufferHandler for App {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl XdgShellHandler for App {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.with_pending_state(|state| {
            let width = self.framebuffer.var_screen_info.xres;
            let height = self.framebuffer.var_screen_info.yres;
            state.states.set(xdg_toplevel::State::Fullscreen);
            state.bounds = Some(Size::from((width, height)));
            state.size = Some(Size::from((width, height)));
        });
        surface.send_configure();
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // Handle popup creation here
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        // Handle popup grab here
    }
}

impl DataDeviceHandler for App {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for App {}
impl ServerDndGrabHandler for App {
    fn send(&mut self, _mime_type: String, _fd: OwnedFd, _seat: Seat<Self>) {}
}

impl CompositorHandler for App {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
    }
}

impl ShmHandler for App {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for App {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}
    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

#[derive(Debug)]
struct App {
    compositor_state: CompositorState,
    xdg_shell_state: XdgShellState,
    shm_state: ShmState,
    seat_state: SeatState<Self>,
    data_device_state: DataDeviceState,
    seat: Seat<Self>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    let mut display: Display<App> = Display::new()?;
    let dh = display.handle();

    let compositor_state = CompositorState::new::<App>(&dh);
    let shm_state = ShmState::new::<App>(&dh, vec![]);
    let mut seat_state = SeatState::new();
    let seat = seat_state.new_wl_seat(&dh, "seat0");

    let mut state = {
        App {
            compositor_state,
            xdg_shell_state: XdgShellState::new::<App>(&dh),
            shm_state,
            seat_state,
            data_device_state: DataDeviceState::new::<App>(&dh),
            seat,
        }
    };

    println!("{state:#?}");

    let listener = ListeningSocket::bind("wayland-0").unwrap();
    let mut clients = Vec::new();
    let (mut renderer, size) = Backend::new();

    loop {
        // There should only be one device, so just get that
        let mut frame = renderer.render((0, 0).into(), Transform::Normal).unwrap();
        let damage = Rectangle::from_loc_and_size((0, 0), size);

        let elements = state
            .xdg_shell_state
            .toplevel_surfaces()
            .iter()
            .flat_map(|surface| {
                render_elements_from_surface_tree(
                    &mut renderer,
                    surface.wl_surface(),
                    (0, 0),
                    1.0,
                    1.0,
                )
            })
            .collect::<Vec<WaylandSurfaceRenderElement<_>>>();

        frame.clear([0.1, 0.0, 0.0, 1.0], &[damage]).unwrap();
        draw_render_elements(&mut frame, 1.0, &elements, &[damage]).unwrap();
        frame.finish().unwrap();

        if let Some(stream) = listener.accept()? {
            println!("Got a client: {:?}", stream);

            let client = display
                .handle()
                .insert_client(stream, Arc::new(ClientState::default()))
                .unwrap();
            clients.push(client);
        }

        display.dispatch_clients(&mut state)?;
        display.flush_clients()?;
    }

    Ok(())
}

#[derive(Default)]
struct ClientState {
    compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {
        println!("initialized");
    }

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        println!("disconnected");
    }
}

// Macros used to delegate protocol handling to types in the app state.
delegate_xdg_shell!(App);
delegate_compositor!(App);
delegate_shm!(App);
delegate_seat!(App);
delegate_data_device!(App);
