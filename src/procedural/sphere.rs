use nalgebra::na;
use nalgebra::na::{Cast, Vec3, Vec2};
use procedural::{MeshDescr, UnifiedIndexBuffer};
use procedural::utils;

/// Generates a UV sphere.
pub fn sphere<N: Float + Cast<f64>>(diameter:      &N,
                                    ntheta_subdiv: u32,
                                    nphi_subdiv:   u32)
                                    -> MeshDescr<N> {
    let mut sphere = unit_sphere(ntheta_subdiv, nphi_subdiv);

    sphere.scale_by_scalar(diameter);

    sphere
}

/// Generates a UV sphere centered at the origin and with a unit diameter.
// FIXME: n{theta,phi}_subdiv are not the right names.
pub fn unit_sphere<N: Float + Cast<f64>>(ntheta_subdiv: u32, nphi_subdiv: u32) -> MeshDescr<N> {
    let two_pi: N = Float::two_pi();
    let pi_two: N = Float::frac_pi_2();
    let dtheta    =  two_pi / na::cast(ntheta_subdiv as f64);
    let dphi      =  pi_two / na::cast(nphi_subdiv as f64);

    let mut coords     = Vec::new();
    let mut curr_phi   = -pi_two + dphi;

    coords.push(-Vec3::y());

    for _ in range(0, 2 * nphi_subdiv - 1) {
        utils::push_circle(curr_phi.cos(), ntheta_subdiv, dtheta, curr_phi.sin(), &mut coords);
        curr_phi = curr_phi + dphi;
    }

    coords.push(Vec3::y());

    // the normals are the same as the coords
    let normals = coords.clone();

    // index buffer
    let mut idx = Vec::new();
    utils::push_degenerate_top_ring_indices(1, 0, ntheta_subdiv, &mut idx);

    for i in range(0, 2 * nphi_subdiv - 2) {
        utils::push_ring_indices(1 + i * ntheta_subdiv, 1 + (i + 1) * ntheta_subdiv, ntheta_subdiv, &mut idx);
    }

    utils::push_degenerate_top_ring_indices(1 + (2 * nphi_subdiv - 2) * ntheta_subdiv,
                                            coords.len() as u32 - 1,
                                            ntheta_subdiv,
                                            &mut idx);
    {
        let len = idx.len();
        utils::reverce_clockwising(idx.mut_slice_from(len - ntheta_subdiv as uint));
    }

    // uvs
    let mut uvs = Vec::new();

    for coord in coords.iter() {
        uvs.push(ball_uv(coord));
    }

    // Result
    let mut out = MeshDescr::new(coords, Some(normals), Some(uvs), Some(UnifiedIndexBuffer(idx)));

    // set the radius to 0.5
    let _0_5: N = na::cast(0.5);
    out.scale_by_scalar(&_0_5);

    out
}

fn ball_uv<N: Float + Cast<f64>>(normal: &Vec3<N>) -> Vec2<N> {
    let two_pi: N = Float::two_pi();
    let pi:     N = Float::pi();
    let _0_5:   N = na::cast(0.5f64);
    let uvx       = _0_5 + normal.z.atan2(&normal.x) / two_pi;
    let uvy       = _0_5 - normal.y.asin() / pi;

    Vec2::new(uvx, uvy)
}

/// Creates an hemisphere with a diameter of 1.
pub fn unit_hemisphere<N: Float + Cast<f64>>(ntheta_subdiv: u32, nphi_subdiv: u32) -> MeshDescr<N> {
    let two_pi: N = Float::two_pi();
    let pi_two: N = Float::frac_pi_2();
    let dtheta    =  two_pi / na::cast(ntheta_subdiv as f64);
    let dphi      =  pi_two / na::cast(nphi_subdiv as f64);

    let mut coords     = Vec::new();
    let mut curr_phi   = na::zero::<N>();

    for _ in range(0, nphi_subdiv - 1) {
        utils::push_circle(curr_phi.cos(), ntheta_subdiv, dtheta, curr_phi.sin(), &mut coords);
        curr_phi = curr_phi + dphi;
    }

    coords.push(Vec3::y());

    let mut idx = Vec::new();

    for i in range(0, nphi_subdiv - 2) {
        utils::push_ring_indices(i * ntheta_subdiv, (i + 1) * ntheta_subdiv, ntheta_subdiv, &mut idx);
    }

    utils::push_degenerate_top_ring_indices((nphi_subdiv - 2) * ntheta_subdiv,
                                            coords.len() as u32 - 1,
                                            ntheta_subdiv,
                                            &mut idx);

    // Result
    let normals = coords.clone();
    // FIXME: uvs
    let mut out = MeshDescr::new(coords, Some(normals), None, Some(UnifiedIndexBuffer(idx)));

    // set the radius to 0.5
    let _0_5: N = na::cast(0.5);
    out.scale_by_scalar(&_0_5);

    out
}