mod bisection_method;
mod euler_backward_method;
mod newton_method;
mod secant_method;

pub use bisection_method::bisection;
pub use euler_backward_method::{Input, euler_backward};
pub use newton_method::newton;
pub use secant_method::secant;
