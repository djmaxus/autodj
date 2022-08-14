//! # AUTOmatic Derivatives & Jacobians
//! pre-alpha publication trial

pub struct DualNumber{
    val: f64,
    eps: f64,
}

impl DualNumber{
    pub fn one() -> Self{
        Self{val: 1.0, eps: 0.0}
    }
    pub fn zero() -> Self{
        Self{val: 1.0, eps: 0.0}
    }
    fn from_deriv(&self, val: f64, deriv: f64) -> Self{
        Self{val, eps: self.eps * deriv}
    }
}

/// binary operations
impl DualNumber{
    pub fn add(self, rhs: Self) ->Self{
        let val = self.val + rhs.val;
        let eps = self.eps + rhs.eps;
        Self{val, eps}
    }

    pub fn sub(self, rhs: Self) ->Self{
        let val = self.val - rhs.val;
        let eps = self.eps - rhs.eps;
        Self{val, eps}
    }

    pub fn mul(self, rhs: Self) ->Self{
        let val = self.val * rhs.val;
        let eps = self.eps *  rhs.val
                +  rhs.eps * self.val;
                Self{val, eps}
    }
    pub fn div(self, rhs: Self) ->Self{
        let val = self.val / rhs.val;
        let eps = (self.eps *  rhs.val
                -   rhs.eps * self.val) / (rhs.val *  rhs.val);
                Self{val, eps}
    }

}

/// unary functions
impl DualNumber{
    pub fn neg(self) ->Self{
        Self{val : -self.val, eps: - self.eps}
    }

    pub fn pow(self, p: f64) ->Self{
        let val = self.val.powf(p);
        let deriv = p * self.val.powf(p-1.);
        self.from_deriv(val, deriv)
    }

    pub fn sin(self) ->Self{
        let (sin, cos) = self.val.sin_cos();
        self.from_deriv(sin, cos)
    }

    pub fn cos(self) ->Self{
        let (sin, cos) = self.val.sin_cos();
        self.from_deriv(cos, -sin)
    }

    pub fn sin_cos(self) ->(Self, Self) {
        let (sin, cos) = self.val.sin_cos();
        (self.from_deriv(sin, cos), self.from_deriv(cos, -sin))
    }

    pub fn exp(self) ->Self{
        let val = self.val.exp();
        self.from_deriv(val,val)
    }

    pub fn ln(self) ->Self{
        let val = self.val.ln();
        let deriv = 1. / self.val;
        self.from_deriv(val, deriv)
    }

    pub fn abs(self) ->Self{
        let val = self.val.abs();
        let deriv = self.val.signum();
        self.from_deriv(val,deriv)
    }

    pub fn signum(self) ->Self{
        let val = self.val.signum();
        self.from_deriv(val,0.0)
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
