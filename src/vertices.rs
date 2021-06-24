use pgl::vao::HasVertexAttributes;
use pgl::GlslDType;

#[repr(C)]
#[derive(Debug)]
pub struct All {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub rotor: [f32; 4],      // tangent to world space rotor (quaternion)
    pub motor: [[f32; 4]; 2], // tangent to world space motor (dual quaternion)
    pub outer_log_motor: [[f32; 3]; 2],
    pub outer_log_rotor: [f32; 3],
    pub qtang: [f32; 3],
    pub cayley_motor: [[f32; 3]; 2],
    pub cayley_rotor: [f32; 3],
}
impl HasVertexAttributes for All {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3, // pos
            GlslDType::Vec2, // uv
            GlslDType::Vec3, // normal
            GlslDType::Vec3, // tangent
            GlslDType::Vec3, // bitangent
            GlslDType::Vec4, // rotor
            GlslDType::Vec4, // motor
            GlslDType::Vec4,
            GlslDType::Vec3, // outer log bivec
            GlslDType::Vec3,
            GlslDType::Vec3, // outer log rotor bivec
            GlslDType::Vec3, // qtangent
            GlslDType::Vec3, // cayley motor
            GlslDType::Vec3,
            GlslDType::Vec3, // cayley rotor
        ]
    }
}
impl From<PosUVNormTang> for All {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize();

        let translator = ppga::Translator::new(&v.position);
        let motor = translator.mul_rotor(&rotor);
        let motor = motor.normalize();

        let outer_log_motor = motor.outer_ln();
        let outer_log_rotor = rotor.outer_ln();

        // let m_ = log_motor.exp();
        // if !m_.is_similar_to(0.001, &motor) {
        //     println!("{:?}", "Mistake in Log Exp");
        //     println!(
        //         "PosByLogMotor {:?}",
        //         m_.apply_to(&ppga::Point::origin()).eucl()
        //     );
        //     println!(
        //         "PosByMotor {:?}",
        //         motor.apply_to(&ppga::Point::origin()).eucl()
        //     );
        //     println!("TruePos {:?}", v.position);
        //     println!("TangentToModelMotor {:?}", motor);
        //     println!("TangentToModelRotor {:?}", rotor);
        //     println!("Exp(Log(TangentToModelMotor)) {:?}", m_);
        //     println!("Tang {:?}", tangent);
        //     println!("BiTang {:?}", bitangent);
        //     println!("Normal {:?}", normal);
        // } else {
        //     println!("{:?}", "For this one Log Exp works!");
        // }

        Self {
            position: v.position,
            uv: v.uv,
            normal: v.normal,
            tangent: v.tangent,
            bitangent: bitangent.into(),
            motor: motor.into_klein(),
            rotor: rotor.into(),
            outer_log_motor: outer_log_motor.into(),
            outer_log_rotor: outer_log_rotor.e_bivector,
            qtang: rotor.qtangent_ln(),
            cayley_motor: motor.cayley_ln().into(),
            cayley_rotor: ppga::Motor::from(&rotor).cayley_ln().e_bivector,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PosUVNormTang {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
}
impl HasVertexAttributes for PosUVNormTang {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3,
            GlslDType::Vec2,
            GlslDType::Vec3,
            GlslDType::Vec3,
        ]
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Matrix {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}
impl HasVertexAttributes for Matrix {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3,
            GlslDType::Vec2,
            GlslDType::Vec3,
            GlslDType::Vec3,
            GlslDType::Vec3,
        ]
    }
}
impl From<PosUVNormTang> for Matrix {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        Self {
            position: v.position,
            uv: v.uv,
            tangent: tangent.into(),
            normal: normal.into(),
            bitangent: bitangent.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Rotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub rotor: [f32; 4], // tangent to world space rotor (quaternion)
}
impl HasVertexAttributes for Rotor {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3, GlslDType::Vec2, GlslDType::Vec4]
    }
}
impl From<PosUVNormTang> for Rotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize()
        .into();

        Self {
            position: v.position,
            uv: v.uv,
            rotor,
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Motor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub motor: [[f32; 4]; 2],
}
impl HasVertexAttributes for Motor {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3,
            GlslDType::Vec2,
            GlslDType::Vec4,
            GlslDType::Vec4,
        ]
    }
}
impl From<PosUVNormTang> for Motor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize()
        .into();

        let t = ppga::Translator::new(&v.position);
        let m = t.mul_rotor(&rotor).normalize();

        Self {
            position: v.position,
            uv: v.uv,
            motor: m.into_klein(),
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct QRotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub outer_rotor: [f32; 3],
}
impl HasVertexAttributes for QRotor {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3, GlslDType::Vec2, GlslDType::Vec3]
    }
}
impl From<PosUVNormTang> for QRotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize();

        Self {
            position: v.position,
            uv: v.uv,
            outer_rotor: rotor.qtangent_ln(),
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CayleyRotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub cayley_rotor: [f32; 3],
}
impl HasVertexAttributes for CayleyRotor {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3, GlslDType::Vec2, GlslDType::Vec3]
    }
}
impl From<PosUVNormTang> for CayleyRotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize();

        Self {
            position: v.position,
            uv: v.uv,
            cayley_rotor: ppga::Motor::from(&rotor).cayley_ln().e_bivector,
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct OuterRotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub outer_rotor: [f32; 3],
}
impl HasVertexAttributes for OuterRotor {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3, GlslDType::Vec2, GlslDType::Vec3]
    }
}
impl From<PosUVNormTang> for OuterRotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize();

        Self {
            position: v.position,
            uv: v.uv,
            outer_rotor: rotor.outer_ln().e_bivector,
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CayleyMotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub cayley_motor: [[f32; 3]; 2],
}
impl HasVertexAttributes for CayleyMotor {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3,
            GlslDType::Vec2,
            GlslDType::Vec3,
            GlslDType::Vec3,
        ]
    }
}
impl From<PosUVNormTang> for CayleyMotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize()
        .into();

        let t = ppga::Translator::new(&v.position);
        let m = t.mul_rotor(&rotor).normalize();

        Self {
            position: v.position,
            uv: v.uv,
            cayley_motor: m.cayley_ln().into(),
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct OuterMotor {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub outer_motor: [[f32; 3]; 2],
}
impl HasVertexAttributes for OuterMotor {
    fn attributes() -> Vec<GlslDType> {
        vec![
            GlslDType::Vec3,
            GlslDType::Vec2,
            GlslDType::Vec3,
            GlslDType::Vec3,
        ]
    }
}
impl From<PosUVNormTang> for OuterMotor {
    fn from(v: PosUVNormTang) -> Self {
        let tangent: glm::Vec3 = v.tangent.into();
        let normal: glm::Vec3 = v.normal.into();
        let bitangent = glm::cross(&normal, &tangent);

        let rotor = ppga::Rotor::from_base(
            &tangent.normalize().into(),
            &bitangent.normalize().into(),
            &normal.normalize().into(),
        )
        .normalize()
        .into();

        let t = ppga::Translator::new(&v.position);
        let m = t.mul_rotor(&rotor).normalize();

        Self {
            position: v.position,
            uv: v.uv,
            outer_motor: m.outer_ln().into(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PosNorm {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl HasVertexAttributes for PosNorm {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3, GlslDType::Vec3]
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PosUVCol {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl HasVertexAttributes for PosUVCol {
    fn attributes() -> Vec<pgl::GlslDType> {
        vec![GlslDType::Vec2, GlslDType::Vec2, GlslDType::Vec4]
    }
}
