//! Can be used for testing the performance of
//! a shader provided in the command line path argument.
//! Prints out a table of results.

use pgl::{
    query::{Query, Target},
    shader::ShaderProgram,
    vao::VertexArray,
    window::GlfwWindow,
};
use prettytable::{Cell, Row, Table};
use pthesis::*;

fn main() {
    let matches = clap::App::new("Shader Bencher")
        .arg(
            clap::Arg::with_name("path")
                .short("p")
                .long("paths")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("n_iter")
                .short("n")
                .long("n_iter")
                .default_value("200")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("width")
                .short("w")
                .long("width")
                .default_value("1600")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("height")
                .short("h")
                .long("height")
                .default_value("1600")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("vertex")
                .short("v")
                .long("vertex")
                .default_value("all")
                .takes_value(true),
        )
        .get_matches();

    let n_iter = matches.value_of("n_iter").unwrap();
    let filepaths = matches.values_of("path").unwrap();
    let width = matches.value_of("width").unwrap().parse().unwrap();
    let height = matches.value_of("height").unwrap().parse().unwrap();
    let vertex = matches.value_of("vertex").unwrap();

    let window = GlfwWindow::new(width, height, "Benchmark");
    let fac = window.hidpi_factor() as isize;
    if fac != 1 {
        window.set_window_size(width / fac, height / fac)
    }
    pgl::utils::gl::set_default_options();

    let nm = pgl::texture::Texture::from_path("imgs/wall_normals.jpeg", Default::default());
    nm.bind_to(1).unwrap();

    let camera = BenchCamera::default();

    let (w, h) = window.framebuffer_size();
    let n_fragments = w * h;
    let grid_size1 = (n_fragments as f32 * 0.8).sqrt() as usize;
    let grid_size2 = (n_fragments as f32 * 0.4).sqrt() as usize;
    let sizes = [grid_size1, grid_size2];
    let benches = match vertex {
        "all" => create_bench_datas::<vertices::All>(&sizes),
        "matrix" => create_bench_datas::<vertices::Matrix>(&sizes),
        "normtang" => create_bench_datas::<vertices::PosUVNormTang>(&sizes),
        "rotor" => create_bench_datas::<vertices::Rotor>(&sizes),
        "outerrotor" => create_bench_datas::<vertices::OuterRotor>(&sizes),
        "qrotor" => create_bench_datas::<vertices::QRotor>(&sizes),
        "motor" => create_bench_datas::<vertices::Motor>(&sizes),
        "outermotor" => create_bench_datas::<vertices::OuterMotor>(&sizes),
        "cayleymotor" => create_bench_datas::<vertices::CayleyMotor>(&sizes),
        "cayleyrotor" => create_bench_datas::<vertices::CayleyRotor>(&sizes),
        _ => unimplemented!(),
    };

    let mut unis = shaders::AppUniforms::new();
    unis.update(&camera, &window, [0., 0., 3.], [0.5, 0.5, 0.5]);
    unis.set_ubo();

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("File"),
        Cell::new("Average Drawtime (ms)"),
        Cell::new("Std"),
        Cell::new("N Indices"),
        Cell::new("N Vertices"),
        Cell::new("N Fragments"),
    ]));
    let exists = std::path::Path::new("results.csv").exists();
    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("results.csv")
        .unwrap();
    let mut csv = csv::WriterBuilder::new()
        .has_headers(if !exists { true } else { false })
        .from_writer(file);

    println!("{:?}", "Starting Benches");

    for filepath in filepaths {
        for data in benches.iter() {
            bench(
                &data,
                &vertex,
                &mut table,
                &mut csv,
                filepath,
                n_iter.parse().unwrap(),
                &window,
            );
        }
    }
    csv.flush().unwrap();
    table.printstd();
}

struct BenchData {
    vao: pgl::vao::VertexArray,
    n_indices: usize,
    n_vertices: usize,
}

fn bench(
    data: &BenchData,
    vertex: &str,
    table: &mut Table,
    csv: &mut csv::Writer<std::fs::File>,
    filename: impl AsRef<std::path::Path>,
    n_frames: usize,
    window: &GlfwWindow,
) {
    let attr_define = match vertex {
        "all" => "ALL",
        "matrix" => "MATRIX_ATTRIBUTES",
        "normtang" => "NORMTANG_ATTRIBUTES",
        "motor" => "MOTOR_ATTRIBUTES",
        "rotor" => "ROTOR_ATTRIBUTES",
        "outermotor" => "OUTER_MOTOR_ATTRIBUTES",
        "outerrotor" => "OUTER_ROTOR_ATTRIBUTES",
        "qrotor" => "QROTOR_ATTRIBUTES",
        "cayleyrotor" => "CAYLEY_ROTOR_ATTRIBUTES",
        "cayleymotor" => "CAYLEY_MOTOR_ATTRIBUTES",
        _ => unimplemented!(),
    };
    let ops = pgl::shader::ShaderOptions {
        vs_defines: vec![attr_define.into()],
        ..Default::default()
    };
    let mut shader = ShaderProgram::from_path(&filename, ops).unwrap();

    let mut draw_times: Vec<i64> = Vec::new();

    shader.bind();
    shader.bind_uniform_block("App", 0);
    shader.set_int("uNormalMap", 1);

    data.vao.bind();

    for _ in 0..n_frames {
        pgl::utils::gl::check_error();
        pgl::utils::gl::flush_error();

        let timer = Query::new(Target::TimeElapsed);
        timer.begin();
        pgl::utils::gl::draw(data.n_indices);
        timer.end();
        draw_times.push(timer.result());

        window.poll_events();
        window.swap_buffers();
        if window.should_close() {
            break;
        }
        pgl::utils::gl::clear();
    }

    let fname = filename.as_ref().file_name().unwrap().to_str().unwrap();
    let skip = 5;
    let draw_times = draw_times
        .iter()
        .skip(skip) // first few frames take longer
        .map(|t| *t as f32 / 1000_000.0)
        .collect::<Vec<_>>();
    let ave_time = draw_times.iter().sum::<f32>() / draw_times.len() as f32;
    let std =
        (draw_times.iter().map(|t| t - ave_time).sum::<f32>() / draw_times.len() as f32).sqrt();
    let (w, h) = window.framebuffer_size();
    let n_frags = w * h;

    table.add_row(Row::new(vec![
        Cell::new(fname),
        Cell::new(&ave_time.to_string()),
        Cell::new(&std.to_string()),
        Cell::new(&data.n_indices.to_string()),
        Cell::new(&data.n_vertices.to_string()),
        Cell::new(&n_frags.to_string()),
    ]));

    csv.serialize(Record {
        filename: fname.to_string(),
        average_drawtime: ave_time,
        std,
        n_indices: data.n_indices,
        n_vertices: data.n_vertices,
        n_fragments: n_frags as usize,
    })
    .unwrap();
}

#[derive(serde::Serialize)]
pub struct Record {
    filename: String,
    average_drawtime: f32,
    std: f32,
    n_vertices: usize,
    n_indices: usize,
    n_fragments: usize,
}

struct BenchCamera {
    pos: glm::Vec3,
}

impl Default for BenchCamera {
    fn default() -> Self {
        Self {
            pos: [0., 4., 0.].into(),
        }
    }
}

impl cameras::Eye for BenchCamera {
    fn view(&self) -> glm::Mat4 {
        glm::look_at::<f32>(&self.pos, &glm::Vec3::zeros(), &-glm::Vec3::z())
    }
    fn projection(&self) -> glm::Mat4 {
        glm::ortho::<f32>(-0.5, 0.5, -0.5, 0.5, 0.1, 20.)
    }
    fn position(&self) -> glm::Vec3 {
        self.pos
    }
}

fn create_bench_datas<V>(grid_sizes: &[usize]) -> Vec<BenchData>
where
    V: 'static + pgl::vao::HasVertexAttributes + From<vertices::PosUVNormTang> + Send,
{
    let (sender, reciever) = std::sync::mpsc::channel();
    for size in grid_sizes.iter() {
        let size = size.clone();
        let s = sender.clone();
        std::thread::spawn(move || {
            let res = grid::<V>(size);
            s.send(res).unwrap();
        });
    }
    drop(sender);
    let mut render_datas = Vec::new();
    while let Ok(data) = reciever.recv() {
        render_datas.push(data);
    }
    render_datas
        .iter()
        .map(|d| {
            let mut vao = VertexArray::new_static();
            vao.bind();
            vao.buffer_indices(&d.1);
            vao.new_vertex_buffer_filled("all", &d.0);
            BenchData {
                vao,
                n_indices: d.1.len() * 3,
                n_vertices: d.0.len(),
            }
        })
        .collect::<Vec<_>>()
}

fn grid<V: From<vertices::PosUVNormTang>>(grid_size: usize) -> (Vec<V>, Vec<pgeom::types::Face>) {
    let (vertices, faces) = pgeom::grid(grid_size as _, grid_size as _);
    let vertices = vertices
        .iter()
        .map(|v| {
            V::from(vertices::PosUVNormTang {
                position: v.position,
                normal: v.normal,
                uv: v.uv,
                tangent: v.tangent,
            })
        })
        .collect::<Vec<_>>();
    (vertices, faces)
}
