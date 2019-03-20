extern crate libc;

use super::Point3d;

pub mod shewchuk {
    extern "C" {
        pub fn exactinit();
        pub fn orient2d(
            pa: *mut libc::c_double,
            pb: *mut libc::c_double,
            pc: *mut libc::c_double,
        ) -> libc::c_double;
        pub fn incircle(
            pa: *mut libc::c_double,
            pb: *mut libc::c_double,
            pc: *mut libc::c_double,
            pp: *mut libc::c_double,
        ) -> libc::c_double;
    }
}

pub fn orient2d(a: &Point3d, b: &Point3d, c: &Point3d, robust_predicates: bool) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    if robust_predicates == true {
        return orient2d_robust(&a, &b, &c);
    } else {
        return orient2d_fast(&a, &b, &c);
    }
}

pub fn orient2d_robust(ppa: &Point3d, ppb: &Point3d, ppc: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let mut a: Vec<f64> = vec![ppa.x, ppa.y];
    let mut b: Vec<f64> = vec![ppb.x, ppb.y];
    let mut c: Vec<f64> = vec![ppc.x, ppc.y];
    let re = unsafe { shewchuk::orient2d(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr()) };
    if re == 0.0 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn orient2d_fast(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let re: f64 = ((a.x - c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x));
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn incircle(a: &Point3d, b: &Point3d, c: &Point3d, p: &Point3d) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    let at = (
        a.x - p.x,
        a.y - p.y,
        (a.x * a.x + a.y * a.y) - (p.x * p.x + p.y * p.y),
    );
    let bt = (
        b.x - p.x,
        b.y - p.y,
        (b.x * b.x + b.y * b.y) - (p.x * p.x + p.y * p.y),
    );
    let ct = (
        c.x - p.x,
        c.y - p.y,
        (c.x * c.x + c.y * c.y) - (p.x * p.x + p.y * p.y),
    );
    let i = at.0 * (bt.1 * ct.2 - bt.2 * ct.1);
    let j = at.1 * (bt.0 * ct.2 - bt.2 * ct.0);
    let k = at.2 * (bt.0 * ct.1 - bt.1 * ct.0);
    let re = i - j + k;
    // println!("INCIRCLE TEST: {}", re);
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}
