use matfile;
use ndarray::IxDyn;
use ndarray::{Array1, Array2, ArrayD};
use std;
use std::convert::TryInto;
use std::error;
use std::fmt;

// Custom Estimation Error
#[derive(PartialEq, Debug)]
enum EstimationError {
    FileNotFound,
    ParseError,
}

impl fmt::Display for EstimationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            EstimationError::FileNotFound => "File not found",
            EstimationError::ParseError => "Parse Error",
        };
        f.write_str(description)
    }
}

impl std::convert::From<std::io::Error> for EstimationError {
    fn from(_: std::io::Error) -> Self {
        EstimationError::FileNotFound
    }
}
impl std::convert::From<matfile::Error> for EstimationError {
    fn from(_: matfile::Error) -> Self {
        EstimationError::ParseError
    }
}

impl error::Error for EstimationError {}

struct BatchEstimator {
    rho_v_c_v: ArrayD<f64>,
    rho_i_pj_i: ArrayD<f64>,
    y_k_j: ArrayD<f64>,
    C_c_v: ArrayD<f64>,
    theta_vk_i: ArrayD<f64>,
    r_i_vk_i: ArrayD<f64>,
    t: ArrayD<f64>,
    w_vk_vk_i: ArrayD<f64>,
    v_vk_vk_i: ArrayD<f64>,
    v_var: ArrayD<f64>,
    w_var: ArrayD<f64>,
    y_var: ArrayD<f64>,
    fu: f64,
    fv: f64,
    cu: f64,
    cv: f64,
    b: f64,
    num_points: i32,
    time_steps: Array1<f64>,
    t_c_v: Array2<f64>,
}

impl BatchEstimator {
    fn new(filename: &str) -> Result<Self, EstimationError> {
        // Read in the file
        let file = std::fs::File::open(filename)?;
        let mat_file = matfile::MatFile::parse(file)?;

        // loading individual elements
        fn load(mat_file: &matfile::MatFile, name: &str) -> Result<ArrayD<f64>, EstimationError> {
            let arr = match mat_file.find_by_name(name) {
                Some(arr) => arr,
                None => return Err(EstimationError::ParseError),
            };
            let arr: ArrayD<f64> = match arr.try_into() {
                Ok(arr) => arr,
                Err(_) => return Err(EstimationError::ParseError),
            };
            Ok(arr)
        }

        // load each field
        let mut batch_estimator = BatchEstimator {
            rho_v_c_v: load(&mat_file, "rho_v_c_v")?,
            rho_i_pj_i: load(&mat_file, "rho_i_pj_i")?,
            y_k_j: load(&mat_file, "y_k_j")?,
            C_c_v: load(&mat_file, "C_c_v")?,
            theta_vk_i: load(&mat_file, "theta_vk_i")?,
            r_i_vk_i: load(&mat_file, "r_i_vk_i")?,
            t: load(&mat_file, "t")?,
            w_vk_vk_i: load(&mat_file, "w_vk_vk_i")?,
            v_vk_vk_i: load(&mat_file, "v_vk_vk_i")?,
            v_var: load(&mat_file, "v_var")?,
            w_var: load(&mat_file, "w_var")?,
            y_var: load(&mat_file, "y_var")?,
            num_points: 20,
            time_steps: Array1::zeros(1900),
            t_c_v: Array2::zeros([4, 4]),
            fu: load(&mat_file, "fu")?[[0, 0]],
            fv: load(&mat_file, "fv")?[[0, 0]],
            cu: load(&mat_file, "cu")?[[0, 0]],
            cv: load(&mat_file, "cv")?[[0, 0]],
            b: load(&mat_file, "b")?[[0, 0]],
        };

        // compute time step difference
        for i in 1..1900 {
            batch_estimator.time_steps[[i]] =
                batch_estimator.t[[0, i]] - batch_estimator.t[[0, i - 1]];
        }

        // make camera to vehicle transformation matrix
        t_c_v;

        Ok(batch_estimator)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let batch_estimator = BatchEstimator::new("data.mat")?;
    Ok(())
}
