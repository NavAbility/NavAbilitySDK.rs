use uuid::Uuid;

use chrono::{
  DateTime, 
  Utc
};

use base64::{
  alphabet,
  engine::{self, general_purpose},
  Engine as _,
};


use crate::entities::Distributions::Distribution;
use crate::{
  entities::Factors::{FactorDFG, FunctionData},
  FullNormal, 
  Point2Point2, 
  Point3Point3, 
  Pose2Pose2, 
  Pose3Pose3, 
  PriorPoint2, 
  PriorPoint3,
  PriorPose2, PriorPose3,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::entities::ClientDFG::NavAbilityDFG;


#[allow(non_snake_case)]
trait FactorType<'a, T: Distribution<'a>> {
  fn new(Z: T) -> Self;
}



impl<'a, T: Distribution<'a>> FactorType<'a, T> for PriorPoint2<T> {
  fn new(Z: T) -> Self {
    Self {
      Z
    }
  }
}

// #[allow(non_snake_case)]
// impl<'a> PriorPoint2<FullNormal<'a>> {
//   pub fn new(Z: FullNormal<'a>) -> Self {
//     Self { Z: Z }
//   }
// }

#[allow(non_snake_case)]
impl<'a> PriorPoint3<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> PriorPose2<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> PriorPose3<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> Point2Point2<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> Point3Point3<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> Pose2Pose2<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}

#[allow(non_snake_case)]
impl<'a> Pose3Pose3<FullNormal<'a>> {
  pub fn new(Z: FullNormal<'a>) -> Self {
    Self { Z: Z }
  }
}



// =======================================================


fn assemble_factor_name(ovlb: Vec<&str>) -> String {
  let mut flb = "".to_string();
  for o in ovlb {
    flb += o;
  }
  flb += "_";
  flb += &(Uuid::new_v4().to_string()[0..4]);
  
  return flb;
}

fn get_fnc_name(fnc: &str) -> String {
  let parts = fnc.split(".");
  let mut t = "";
  for part in parts {
    t = part;
  }
  return t.to_owned();
}

impl FunctionData {
  pub fn new(
    fnc: &str,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>,
  ) -> Self {
    let mut fd = Self::default();
    fd.fnc = fnc.to_owned();
    fd.nullhypo = nullhypo.unwrap_or(0.0);
    fd.multihypo = multihypo.unwrap_or(Vec::new());
    fd.inflation = inflation.unwrap_or(3.0);
    return fd;
  }
  
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap().to_string()
  }
  
  pub fn to_string_b64(&self) -> String {
    general_purpose::STANDARD.encode(self.to_json().to_string())
  }
}

impl FactorDFG {
  /// Create a new factor
  /// # Arguments
  /// * `varlbls` - variable labels is a ordered vector of variable label strings, e.g. ["x1", "x2"]
  /// * `fnctype` - function type, e.g. "RoME.Pose3Pose3", "RoME.PriorPose2", see Caesar.jl.
  /// * `tags` - tags are additional strings to help identify the factor
  /// * `timestamp` - timestamp, in DateTime<Utc>, default is now
  /// * `nstime` - nstime, also known as "time since epoch" in nanoseconds
  /// * # Returns
  /// * `FactorDFG` - a new factor
  /// # Example
  /// ```
  /// use navabilitysdk::services::Factors;
  /// use chrono::{DateTime, Utc};
  /// let f = Factors::new(vec!["x1", "x2"], "RoME.Pose3Pose3", vec!["ODOMETRY","BODY_FRAME"], Some(Utc::now()));
  /// ```
  /// # Note
  /// * This is a simplified version of the original function, which is more complex and has more options.
  pub fn new(
    varlbls: Vec<&str>,
    fnctype: &str,
    tags: Vec<&str>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
  ) -> Self {
    return Self::new_more(
      varlbls, fnctype, tags, timestamp, nstime, None, None, None, None,
    );
  }
  
  /// Create a new factor with more expansive inputs.  See `new` for details.
  /// # Arguments
  /// solvable - solvable, default is 1 which means the solver will attempt compute numerical values
  /// multihypo - multihypo, default is empty vector.  Allows fractional hypotheses, see Caesar.jl Docs for details
  /// nullhypo - nullhypo, default is 0.0.  Allows fractional null hypotheses, see Caesar.jl Docs for details
  /// inflation - inflation, default is 3.0.  Allows inflation of the covariance, see Caesar.jl Docs for details
  /// # Returns
  /// * `FactorDFG` - a new factor
  pub fn new_more(
    varlbls: Vec<&str>,
    fnctype: &str,
    tags: Vec<&str>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
    solvable: Option<i64>,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>,
  ) -> Self {
    let mut f = FactorDFG::default();
    
    for vl in varlbls {
      f.variableOrderSymbols_.push(vl.to_string());
    }
    f.fnctype = get_fnc_name(fnctype);
    f.tags.push("FACTOR".to_string());
    for t in tags {
      if !t.eq("FACTOR") {
        f.tags.push(t.to_string());
      }
    }
    // default on create, also deser is different use-case
    f.timestamp = Some(timestamp.unwrap_or(Utc::now()));
    f.nstime = Some(format!("{}", nstime.unwrap_or(0)));
    f.solvable = Some(solvable.unwrap_or(0));
    
    let fdata = FunctionData::new(&f.fnctype.to_string(), multihypo, nullhypo, inflation);
    // FIXME, should not be json'd so early: JuliaRobotics/DistributedFactorGraphs.jl#1118
    f.data = Some(fdata.to_json());
    
    return f;
  }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn post_add_factor(
  nvacl: &NavAbilityDFG,
  factor: FactorDFG,
) {
  todo!()
}



// function DFG.addFactor!(
//     fgclient::NavAbilityDFG,
//     xisyms::Vector{Symbol},
//     fnc::InferenceType;
//     multihypo::Vector{Float64} = Float64[],
//     nullhypo::Float64 = 0.0,
//     solvable::Int = 1,
//     tags::Vector{Symbol} = Symbol[],
//     timestamp::ZonedDateTime = TimeZones.now(tz"UTC"),
//     inflation::Real = 3.0,
//     label::Symbol = assembleFactorName(xisyms),
//     nstime::Int = 0,
//     metadata::Dict{Symbol, DFG.SmallDataTypes} = Dict{Symbol, DFG.SmallDataTypes}(),
// )
//     # create factor data
//     factordata = FactorData(; fnc, multihypo, nullhypo, inflation)

//     fnctype = getFncTypeName(fnc)

//     union!(tags, [:FACTOR])
//     # create factor
//     factor = FactorDFG(;
//         label,
//         tags,
//         _variableOrderSymbols = xisyms,
//         timestamp,
//         nstime = string(nstime),
//         fnctype,
//         solvable,
//         data = JSON3.write(factordata),
//         metadata = base64encode(JSON3.write(metadata)),
//     )

//     # add factor
//     resultId = addFactor!(fgclient, factor)

//     return resultId
// end
