use libipld::ipld::Ipld;

use std::{
  convert::TryFrom,
  fmt,
};

use crate::{
  ipld_error::IpldError,
  literal::Literal,
  prim::bits,
  term::Term,
  yatima,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum U16Op {
  Max,
  Min,
  Eql,
  Lte,
  Lth,
  Gth,
  Gte,
  Not,
  And,
  Or,
  Xor,
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,
  Shl,
  Shr,
  Rol,
  Ror,
  CountZeros,
  CountOnes,
  ToU8,
  ToU32,
  ToU64,
  ToU128,
  ToNat,
  ToI8,
  ToI16,
  ToI32,
  ToI64,
  ToI128,
  ToInt,
  ToBits,
  ToBytes,
}

impl U16Op {
  pub fn symbol(self) -> String {
    match self {
      Self::Max => "max".to_owned(),
      Self::Min => "min".to_owned(),
      Self::Eql => "eql".to_owned(),
      Self::Lte => "lte".to_owned(),
      Self::Lth => "lth".to_owned(),
      Self::Gth => "gth".to_owned(),
      Self::Gte => "gte".to_owned(),
      Self::Not => "not".to_owned(),
      Self::And => "and".to_owned(),
      Self::Or => "or".to_owned(),
      Self::Xor => "xor".to_owned(),
      Self::Add => "add".to_owned(),
      Self::Sub => "sub".to_owned(),
      Self::Mul => "mul".to_owned(),
      Self::Div => "div".to_owned(),
      Self::Mod => "mod".to_owned(),
      Self::Pow => "pow".to_owned(),
      Self::Shl => "shl".to_owned(),
      Self::Shr => "shr".to_owned(),
      Self::Rol => "rol".to_owned(),
      Self::Ror => "ror".to_owned(),
      Self::CountZeros => "count_zeros".to_owned(),
      Self::CountOnes => "count_ones".to_owned(),
      Self::ToU8 => "to_U8".to_owned(),
      Self::ToU32 => "to_U32".to_owned(),
      Self::ToU64 => "to_U64".to_owned(),
      Self::ToU128 => "to_U128".to_owned(),
      Self::ToNat => "to_Nat".to_owned(),
      Self::ToI8 => "to_I8".to_owned(),
      Self::ToI16 => "to_I16".to_owned(),
      Self::ToI32 => "to_I32".to_owned(),
      Self::ToI64 => "to_I64".to_owned(),
      Self::ToI128 => "to_I128".to_owned(),
      Self::ToInt => "to_Int".to_owned(),
      Self::ToBytes => "to_Bytes".to_owned(),
      Self::ToBits => "to_Bits".to_owned(),
    }
  }

  pub fn from_symbol(x: &str) -> Option<Self> {
    match x {
      "max" => Some(Self::Max),
      "min" => Some(Self::Min),
      "eql" => Some(Self::Eql),
      "lte" => Some(Self::Lte),
      "lth" => Some(Self::Lth),
      "gth" => Some(Self::Gth),
      "gte" => Some(Self::Gte),
      "not" => Some(Self::Not),
      "and" => Some(Self::And),
      "or" => Some(Self::Or),
      "xor" => Some(Self::Xor),
      "add" => Some(Self::Add),
      "sub" => Some(Self::Sub),
      "mul" => Some(Self::Mul),
      "div" => Some(Self::Div),
      "mod" => Some(Self::Mod),
      "pow" => Some(Self::Pow),
      "shl" => Some(Self::Shl),
      "shr" => Some(Self::Shr),
      "rol" => Some(Self::Rol),
      "ror" => Some(Self::Ror),
      "count_zeros" => Some(Self::CountZeros),
      "count_ones" => Some(Self::CountOnes),
      "to_U8" => Some(Self::ToU8),
      "to_U32" => Some(Self::ToU32),
      "to_U64" => Some(Self::ToU64),
      "to_U128" => Some(Self::ToU128),
      "to_Nat" => Some(Self::ToNat),
      "to_I8" => Some(Self::ToI8),
      "to_I16" => Some(Self::ToI16),
      "to_I32" => Some(Self::ToI32),
      "to_I64" => Some(Self::ToI64),
      "to_I128" => Some(Self::ToI128),
      "to_Int" => Some(Self::ToInt),
      "to_Bytes" => Some(Self::ToBytes),
      "to_Bits" => Some(Self::ToBits),
      _ => None,
    }
  }

  pub fn type_of(self) -> Term {
    match self {
      Self::Max => yatima!("#U16"),
      Self::Min => yatima!("#U16"),
      Self::Eql => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Lte => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Lth => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Gth => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Gte => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Not => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::And => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Or => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Xor => yatima!("∀ #U16 #U16 -> #Bool"),
      Self::Add => yatima!("∀ #U16 #U16 -> #U16"),
      Self::Sub => yatima!("∀ #U16 #U16 -> #U16"),
      Self::Mul => yatima!("∀ #U16 #U16 -> #U16"),
      Self::Div => yatima!("∀ #U16 #U16 -> #U16"),
      Self::Mod => yatima!("∀ #U16 #U16 -> #U16"),
      Self::Pow => yatima!("∀ #U16 #U32 -> #U16"),
      Self::Shl => yatima!("∀ #U32 #U16 -> #U16"),
      Self::Shr => yatima!("∀ #U32 #U16 -> #U16"),
      Self::Rol => yatima!("∀ #U32 #U16 -> #U16"),
      Self::Ror => yatima!("∀ #U32 #U16 -> #U16"),
      Self::CountZeros => yatima!("∀ #U16 -> #U32"),
      Self::CountOnes => yatima!("∀ #U16 -> #U32"),
      Self::ToU8 => yatima!("∀ #U16 -> #U8"),
      Self::ToU32 => yatima!("∀ #U16 -> #U32"),
      Self::ToU64 => yatima!("∀ #U16 -> #U64"),
      Self::ToU128 => yatima!("∀ #U16 -> #U128"),
      Self::ToNat => yatima!("∀ #U16 -> #Nat"),
      Self::ToI8 => yatima!("∀ #U16 -> #I8"),
      Self::ToI16 => yatima!("∀ #U16 -> #I16"),
      Self::ToI32 => yatima!("∀ #U16 -> #I32"),
      Self::ToI64 => yatima!("∀ #U16 -> #I64"),
      Self::ToI128 => yatima!("∀ #U16 -> #I128"),
      Self::ToInt => yatima!("∀ #U16 -> #Int"),
      Self::ToBytes => yatima!("∀ #U16 -> #Bytes"),
      Self::ToBits => yatima!("∀ #U8 -> #Bits"),
    }
  }

  pub fn to_ipld(self) -> Ipld {
    match self {
      Self::Max => Ipld::Integer(0),
      Self::Min => Ipld::Integer(1),
      Self::Eql => Ipld::Integer(2),
      Self::Lte => Ipld::Integer(3),
      Self::Lth => Ipld::Integer(4),
      Self::Gth => Ipld::Integer(5),
      Self::Gte => Ipld::Integer(6),
      Self::Not => Ipld::Integer(7),
      Self::And => Ipld::Integer(8),
      Self::Or => Ipld::Integer(9),
      Self::Xor => Ipld::Integer(10),
      Self::Add => Ipld::Integer(11),
      Self::Sub => Ipld::Integer(12),
      Self::Mul => Ipld::Integer(13),
      Self::Div => Ipld::Integer(14),
      Self::Mod => Ipld::Integer(15),
      Self::Pow => Ipld::Integer(16),
      Self::Shl => Ipld::Integer(17),
      Self::Shr => Ipld::Integer(18),
      Self::Rol => Ipld::Integer(19),
      Self::Ror => Ipld::Integer(20),
      Self::CountZeros => Ipld::Integer(21),
      Self::CountOnes => Ipld::Integer(22),
      Self::ToU8 => Ipld::Integer(23),
      Self::ToU32 => Ipld::Integer(24),
      Self::ToU64 => Ipld::Integer(25),
      Self::ToU128 => Ipld::Integer(26),
      Self::ToNat => Ipld::Integer(27),
      Self::ToI8 => Ipld::Integer(28),
      Self::ToI16 => Ipld::Integer(29),
      Self::ToI32 => Ipld::Integer(30),
      Self::ToI64 => Ipld::Integer(31),
      Self::ToI128 => Ipld::Integer(32),
      Self::ToInt => Ipld::Integer(33),
      Self::ToBits => Ipld::Integer(34),
      Self::ToBytes => Ipld::Integer(35),
    }
  }

  pub fn from_ipld(ipld: &Ipld) -> Result<Self, IpldError> {
    match ipld {
      Ipld::Integer(0) => Ok(Self::Max),
      Ipld::Integer(1) => Ok(Self::Min),
      Ipld::Integer(2) => Ok(Self::Eql),
      Ipld::Integer(3) => Ok(Self::Lte),
      Ipld::Integer(4) => Ok(Self::Lth),
      Ipld::Integer(5) => Ok(Self::Gth),
      Ipld::Integer(6) => Ok(Self::Gte),
      Ipld::Integer(7) => Ok(Self::Not),
      Ipld::Integer(8) => Ok(Self::And),
      Ipld::Integer(9) => Ok(Self::Or),
      Ipld::Integer(10) => Ok(Self::Xor),
      Ipld::Integer(11) => Ok(Self::Add),
      Ipld::Integer(12) => Ok(Self::Sub),
      Ipld::Integer(13) => Ok(Self::Mul),
      Ipld::Integer(14) => Ok(Self::Div),
      Ipld::Integer(15) => Ok(Self::Mod),
      Ipld::Integer(16) => Ok(Self::Pow),
      Ipld::Integer(17) => Ok(Self::Shl),
      Ipld::Integer(18) => Ok(Self::Shr),
      Ipld::Integer(19) => Ok(Self::Rol),
      Ipld::Integer(20) => Ok(Self::Ror),
      Ipld::Integer(21) => Ok(Self::CountZeros),
      Ipld::Integer(22) => Ok(Self::CountOnes),
      Ipld::Integer(23) => Ok(Self::ToU8),
      Ipld::Integer(24) => Ok(Self::ToU32),
      Ipld::Integer(25) => Ok(Self::ToU64),
      Ipld::Integer(26) => Ok(Self::ToU128),
      Ipld::Integer(27) => Ok(Self::ToNat),
      Ipld::Integer(28) => Ok(Self::ToI8),
      Ipld::Integer(29) => Ok(Self::ToI16),
      Ipld::Integer(30) => Ok(Self::ToI32),
      Ipld::Integer(31) => Ok(Self::ToI64),
      Ipld::Integer(32) => Ok(Self::ToI128),
      Ipld::Integer(33) => Ok(Self::ToInt),
      Ipld::Integer(34) => Ok(Self::ToBits),
      Ipld::Integer(35) => Ok(Self::ToBytes),
      xs => Err(IpldError::NatOp(xs.to_owned())),
    }
  }

  pub fn arity(self) -> u64 {
    match self {
      Self::Max => 0,
      Self::Min => 0,
      Self::Eql => 2,
      Self::Lte => 2,
      Self::Lth => 2,
      Self::Gth => 2,
      Self::Gte => 2,
      Self::Not => 1,
      Self::And => 2,
      Self::Or => 2,
      Self::Xor => 2,
      Self::Add => 2,
      Self::Sub => 2,
      Self::Mul => 2,
      Self::Div => 2,
      Self::Mod => 2,
      Self::Pow => 2,
      Self::Shl => 2,
      Self::Shr => 2,
      Self::Rol => 2,
      Self::Ror => 2,
      Self::CountZeros => 1,
      Self::CountOnes => 1,
      Self::ToU8 => 1,
      Self::ToU32 => 1,
      Self::ToU64 => 1,
      Self::ToU128 => 1,
      Self::ToNat => 1,
      Self::ToI8 => 1,
      Self::ToI16 => 1,
      Self::ToI32 => 1,
      Self::ToI64 => 1,
      Self::ToI128 => 1,
      Self::ToInt => 1,
      Self::ToBytes => 1,
      Self::ToBits => 1,
    }
  }

  pub fn apply0(self) -> Option<Literal> {
    use Literal::*;
    match self {
      Self::Max => Some(U16(u16::MAX)),
      Self::Min => Some(U16(u16::MIN)),
      _ => None,
    }
  }

  pub fn apply1(self, x: Literal) -> Option<Literal> {
    use Literal::*;
    match (self, x) {
      (Self::CountZeros, U16(x)) => Some(U32(x.count_zeros())),
      (Self::CountOnes, U16(x)) => Some(U32(x.count_ones())),
      (Self::ToU8, U16(x)) => u16::try_from(x).ok().map(U16),
      (Self::ToU32, U16(x)) => Some(U32(x.into())),
      (Self::ToU64, U16(x)) => Some(U64(x.into())),
      (Self::ToU128, U16(x)) => Some(U128(x.into())),
      (Self::ToI8, U16(x)) => i8::try_from(x).ok().map(I8),
      (Self::ToI16, U16(x)) => i16::try_from(x).ok().map(I16),
      (Self::ToI32, U16(x)) => Some(I32(x.into())),
      (Self::ToI64, U16(x)) => Some(I64(x.into())),
      (Self::ToI128, U16(x)) => Some(I128(x.into())),
      (Self::Not, U16(x)) => Some(U16(!x)),
      (Self::ToInt, U16(x)) => Some(Int(x.into())),
      (Self::ToBytes, U16(x)) => Some(Bytes(x.to_be_bytes().into())),
      (Self::ToBits, U16(x)) => {
        Some(Bits(bits::bytes_to_bits(16, x.to_be_bytes().into())))
      }
      _ => None,
    }
  }

  pub fn apply2(self, x: Literal, y: Literal) -> Option<Literal> {
    use Literal::*;
    match (self, x, y) {
      (Self::Eql, U16(x), U16(y)) => Some(Bool(x == y)),
      (Self::Lte, U16(x), U16(y)) => Some(Bool(x <= y)),
      (Self::Lth, U16(x), U16(y)) => Some(Bool(x < y)),
      (Self::Gth, U16(x), U16(y)) => Some(Bool(x > y)),
      (Self::Gte, U16(x), U16(y)) => Some(Bool(x >= y)),
      (Self::And, U16(x), U16(y)) => Some(U16(x & y)),
      (Self::Or, U16(x), U16(y)) => Some(U16(x | y)),
      (Self::Xor, U16(x), U16(y)) => Some(U16(x ^ y)),
      (Self::Add, U16(x), U16(y)) => Some(U16(x.wrapping_add(y))),
      (Self::Sub, U16(x), U16(y)) => Some(U16(x.wrapping_sub(y))),
      (Self::Mul, U16(x), U16(y)) => Some(U16(x.wrapping_mul(y))),
      (Self::Div, U16(x), U16(y)) => Some(U16(x.wrapping_div(y))),
      (Self::Mod, U16(x), U16(y)) => Some(U16(x.wrapping_rem(y))),
      (Self::Pow, U16(x), U32(y)) => Some(U16(x.wrapping_pow(y))),
      (Self::Shl, U32(x), U16(y)) => Some(U16(y.wrapping_shl(x))),
      (Self::Shr, U32(x), U16(y)) => Some(U16(y.wrapping_shr(x))),
      (Self::Rol, U32(x), U16(y)) => Some(U16(y.rotate_left(x))),
      (Self::Ror, U32(x), U16(y)) => Some(U16(y.rotate_right(x))),
      _ => None,
    }
  }
}

impl fmt::Display for U16Op {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.symbol())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use quickcheck::{
    Arbitrary,
    Gen,
  };
  use rand::Rng;
  impl Arbitrary for U16Op {
    fn arbitrary(_g: &mut Gen) -> Self {
      let mut rng = rand::thread_rng();
      let gen: u32 = rng.gen_range(0..34);
      match gen {
        0 => Self::Max,
        1 => Self::Min,
        2 => Self::Eql,
        3 => Self::Lte,
        4 => Self::Lth,
        5 => Self::Gth,
        6 => Self::Gte,
        7 => Self::Not,
        8 => Self::And,
        9 => Self::Or,
        10 => Self::Xor,
        11 => Self::Add,
        12 => Self::Sub,
        13 => Self::Mul,
        14 => Self::Div,
        15 => Self::Mod,
        16 => Self::Pow,
        17 => Self::Shl,
        18 => Self::Shr,
        19 => Self::Rol,
        20 => Self::Ror,
        21 => Self::CountZeros,
        22 => Self::CountOnes,
        23 => Self::ToU8,
        24 => Self::ToU32,
        25 => Self::ToU64,
        26 => Self::ToU128,
        27 => Self::ToNat,
        28 => Self::ToI8,
        29 => Self::ToI16,
        30 => Self::ToI32,
        31 => Self::ToI64,
        32 => Self::ToI128,
        33 => Self::ToInt,
        _ => Self::ToBytes,
      }
    }
  }

  #[quickcheck]
  fn u16_op_ipld(x: U16Op) -> bool {
    match U16Op::from_ipld(&x.to_ipld()) {
      Ok(y) => x == y,
      _ => false,
    }
  }
}