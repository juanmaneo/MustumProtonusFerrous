//! glu32 subset for MPF: the deterministic logic of Wine's `dlls/glu32/geom.c` and
//! `dlls/glu32/project.c`.
//!
//! Ported (table tests against a verbatim Wine C oracle, see `oracle_checks.rs`):
//! - Vertex predicates: `vert_eq`, `vert_leq`, `trans_leq` and `vert_ccw`.
//! - Edge arithmetic: `edge_eval`, `edge_sign`, `trans_eval`, `trans_sign` and
//!   `edge_intersect`, which uses the `Interpolate` weighting of `geom.c`.
//! - Matrix helpers: `mult_matrix_vec`, `mult_matrices` and `invert_matrix`.
//! - `project` (`gluProject`) and `unproject` (`gluUnProject`).
//!
//! Divergences from the C bodies:
//! - `edge_intersect` returns the vertex, where C writes through its `v` out parameter.
//!   `sweep.c` passes a separate `isect`, so the aliasing case never arises.
//! - `GL_TRUE` and `GL_FALSE` results are `Option`s: `None` where C returns `GL_FALSE`
//!   and leaves the output unwritten.
//! - The C `assert`s in the edge functions are `assert!`s, so a broken ordering
//!   precondition panics.
//! - Degenerate numbers follow IEEE arithmetic, as in C. A zero viewport size or a NaN
//!   yields an infinity or NaN and does not panic.
//!
//! Not ported:
//! - The rest of the tessellator: `mesh.c`, `tess.c`, `sweep.c` and `priorityq.c` use a
//!   half-edge mesh and a heap with allocation handles.
//! - `mipmap.c`, which depends on GL pixel-store state.
//! - `quad.c` and `render.c`, which emit GL calls.
//! - `glu.c`, which maps GL and GLU error codes to strings.
//! - `gluOrtho2D`, `gluPerspective`, `gluLookAt` and `gluPickMatrix`, which only issue
//!   GL state calls.

mod geom;
mod project;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use geom::{
    edge_eval, edge_intersect, edge_sign, trans_eval, trans_leq, trans_sign, vert_ccw, vert_eq,
    vert_leq, Vertex,
};
pub use project::{invert_matrix, mult_matrices, project, unproject};

/// `__gluMultMatrixVecd`: multiplies a column-major matrix by a vector.
pub fn mult_matrix_vec(matrix: &[f64; 16], input: &[f64; 4]) -> [f64; 4] {
    std::array::from_fn(|i| {
        input[0] * matrix[i]
            + input[1] * matrix[4 + i]
            + input[2] * matrix[8 + i]
            + input[3] * matrix[12 + i]
    })
}
