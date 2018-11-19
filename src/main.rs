use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use termesh::drawille::Canvas;
use termesh::stl::Stl;

/// Display 3d objects in the terminal using Braille characters.
#[derive(Debug, StructOpt)]
struct App {
    #[structopt(subcommand)]
    command: Option<Command>,

    /// Input mesh to display. Only binary STL as of now.
    #[structopt(short = "m", long = "mesh", parse(from_os_str))]
    mesh_filepath: PathBuf,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Interactively render an input mesh in the terminal.
    #[structopt(name = "render")]
    Render(RenderConfig),

    /// Dump an input mesh in the terminal.
    #[structopt(name = "dump")]
    Dump(DumpConfig),
}

#[derive(Debug, StructOpt)]
struct RenderConfig {
    /// Do not render using true colors. This will effectively make the depth
    /// all the same.
    #[structopt(long = "no-colors")]
    no_colors: bool,

    /// Display only the wireframe of the mesh.
    #[structopt(short = "w", long = "wireframe")]
    only_wireframe: bool,
}

#[derive(Debug, StructOpt)]
struct DumpConfig {
    /// Scale the input mesh by a given factor before dumping.
    #[structopt(short = "s", long = "scale", default_value = "1")]
    scale: f32,

    /// Rotate the input mesh around the x axis by a given angle in radians
    /// before dumping.
    #[structopt(short = "x", long = "rotation-x", default_value = "0")]
    rotation_x: f32,

    /// Rotate the input mesh around the y axis by a given angle in radians
    /// before dumping.
    #[structopt(short = "y", long = "rotation-y", default_value = "0")]
    rotation_y: f32,

    /// Rotate the input mesh around the z axis by a given angle in radians
    /// before dumping.
    #[structopt(short = "z", long = "rotation-z", default_value = "0")]
    rotation_z: f32,
}

fn main() -> io::Result<()> {
    let app = App::from_args();

    let mut f = File::open(app.mesh_filepath)?;
    let stl = Stl::parse_binary(&mut f)?;

    let default_command = Command::Render(RenderConfig {
        no_colors: false,
        only_wireframe: false,
    });

    match app.command.unwrap_or(default_command) {
        Command::Render(config) => {
            if termion::is_tty(&io::stdout()) {
                interactive(config, stl)
            } else {
                non_interactive(
                    DumpConfig {
                        scale: 1.0,
                        rotation_x: 0.0,
                        rotation_y: 0.0,
                        rotation_z: 0.0,
                    },
                    stl,
                )
            }
        }
        Command::Dump(config) => non_interactive(config, stl),
    }
}

fn non_interactive(config: DumpConfig, mut stl: Stl) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    rotate_stl(
        &mut stl,
        config.rotation_x,
        config.rotation_y,
        config.rotation_z,
    );
    scale_stl(&mut stl, config.scale);

    render_stl(
        &mut stdout,
        &stl,
        false,
        None,
        &RenderConfig {
            no_colors: true,
            only_wireframe: true,
        },
    )?;

    Ok(())
}

fn interactive(config: RenderConfig, stl: Stl) -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode()?;
    // let mut stdout = io::stdout();
    write!(stdout, "{}\r\n", termion::cursor::Hide)?;

    let mut scale_and_draw = |mut stl| -> io::Result<()> {
        let terminal_size = termion::terminal_size()?;

        let padding = 5;
        let scale =
            determine_scale_factor(&stl, terminal_size.0 - padding, terminal_size.1 - padding);

        scale_stl(&mut stl, scale);
        render_stl(
            &mut stdout,
            &stl,
            true,
            Some((i32::from(terminal_size.0), i32::from(terminal_size.1))),
            &config,
        )?;

        Ok(())
    };

    scale_and_draw(stl.clone())?;

    let angle_inc = PI / 6.0;

    let mut rotation_x = 0.0;
    let mut rotation_y = 0.0;
    let mut rotation_z = 0.0;

    for ev in io::stdin().keys() {
        let ev = ev?;

        match ev {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Char('x') => {
                rotation_x = (rotation_x + angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('X') => {
                rotation_x = (rotation_x - angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('y') => {
                rotation_y = (rotation_y + angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('Y') => {
                rotation_y = (rotation_y - angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('z') => {
                rotation_z = (rotation_z + angle_inc) % (2.0 * PI);
            }
            termion::event::Key::Char('Z') => {
                rotation_z = (rotation_z - angle_inc) % (2.0 * PI);
            }
            _ => continue,
        }

        let mut stl = stl.clone();
        rotate_stl(&mut stl, rotation_x, rotation_y, rotation_z);
        scale_and_draw(stl)?;
    }

    write!(
        stdout,
        "{}{}{}{}\r\n",
        termion::color::Bg(termion::color::Reset),
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;

    Ok(())
}

fn rotate_stl(stl: &mut Stl, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
    if rotation_x == 0.0 && rotation_y == 0.0 && rotation_z == 0.0 {
        return;
    }

    for v in stl.vertices_mut() {
        if rotation_x != 0.0 {
            v.rotate_x(rotation_x);
        }

        if rotation_y != 0.0 {
            v.rotate_y(rotation_y);
        }

        if rotation_z != 0.0 {
            v.rotate_z(rotation_z);
        }
    }
}

fn scale_stl(stl: &mut Stl, scale: f32) {
    if scale == 1.0 {
        return;
    }

    for v in stl.vertices_mut() {
        *v *= scale;
    }
}

fn render_stl<W: Write>(
    w: &mut W,
    stl: &Stl,
    clear: bool,
    max_dimensions: Option<(i32, i32)>,
    render_config: &RenderConfig,
) -> io::Result<()> {
    let mut canvas = Canvas::new();

    if render_config.only_wireframe {
        for f in &stl.facets {
            canvas.triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
        }
    } else {
        for f in &stl.facets {
            canvas.fill_triangle(f.vertices[0], f.vertices[1], f.vertices[2]);
        }
    }

    // callers can clear the screen by themselves, but it usually causes
    // flickering on big terminals. Therefore defer clearing the screen until
    // the very last.
    if clear {
        // changing the background color needs clearing before it can be
        // rendered effectively
        if !render_config.no_colors {
            write!(
                w,
                "{}",
                termion::color::Bg(termesh::drawille::Canvas::background_color())
            )?;
        }
        clear_screen(w)?;
    }

    match max_dimensions {
        None => {
            for r in canvas.rows(!render_config.no_colors) {
                write!(w, "{}\r\n", r)?;
            }
            w.flush()
        }
        Some((max_width, max_height)) => {
            if let Some((min_r, max_r, min_c, max_c)) = canvas.dimensions() {
                let padded = |min, max, max_len| {
                    if max_len <= max - min {
                        min
                    } else {
                        let padding = max_len - (max - min);
                        min - padding / 2
                    }
                };

                let min_r = padded(min_r, max_r, max_height);
                let min_c = padded(min_c, max_c, max_width);

                for r in canvas.frame(!render_config.no_colors, min_r, max_r, min_c, max_c) {
                    write!(w, "{}\r\n", r)?;
                }
                w.flush()?;
            }

            Ok(())
        }
    }
}

fn determine_scale_factor(stl: &Stl, max_width: u16, max_height: u16) -> f32 {
    let mut vs = stl.vertices();

    let (w, h) = vs
        .next()
        .map(|v| {
            vs.fold((v.x, v.y, v.x, v.y), |(min_x, min_y, max_x, max_y), v| {
                (
                    min_x.min(v.x),
                    min_y.min(v.y),
                    max_x.max(v.x),
                    max_y.max(v.y),
                )
            })
        })
        .map_or((1.0, 1.0), |(min_x, min_y, max_x, max_y)| {
            (max_x - min_x, max_y - min_y)
        });

    let scalex = f32::from(max_width) / w * 2.0;
    let scaley = f32::from(max_height) / h * 4.0;

    scalex.min(scaley)
}

fn clear_screen<W: Write>(w: &mut W) -> io::Result<()> {
    write!(
        w,
        "{}{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )?;
    w.flush()?;

    Ok(())
}
