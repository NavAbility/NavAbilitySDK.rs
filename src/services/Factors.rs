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


use crate::{entities::Distributions::Distribution, FullNormal, SDK_VERSION};
use crate::{
  entities::Factors::{FactorDFG, FunctionData},
  // FullNormal, 
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


// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! GenDistrFactor { 
  ($T:ident) => {
    impl<'a, D: Distribution<'a>> crate::FactorType<'a, D> for $T<D> {
      fn new(Z: D) -> Self {
        Self {
          Z
        }
      }
    }
  }
}


GenDistrFactor!(PriorPoint2);
GenDistrFactor!(PriorPoint3);
GenDistrFactor!(PriorPose2);
GenDistrFactor!(PriorPose3);
GenDistrFactor!(Point2Point2);
GenDistrFactor!(Point3Point3);
GenDistrFactor!(Pose2Pose2);
GenDistrFactor!(Pose3Pose3);



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
    general_purpose::STANDARD.encode(self.to_json())
  }
}


// TODO support more D: Distributions<'a>
impl<'a, F> FactorDFG<F> 
where 
  F: crate::FactorType<'a, FullNormal<'a>>
{
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
  /// let f = Factors::new(
  ///   vec!["x1", "x2"], 
  ///   Pose2Pose2::new(FullNormal::new(vec![&[1.0, 2.0, 3.0], &[0.01, 0.01, 0.01]])),
  ///   vec!["ODOMETRY","BODY_MOTION"], 
  ///   Some(Utc::now())
  /// );
  /// ```
  /// # Note
  /// * This is a simplified version of the ::new_more function, which has more options.
  pub fn new(
    varlbls: Vec<&str>,
    fnctype: F,
    tags: Vec<&str>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
  ) -> Self {
    return Self::new_more(
      varlbls, 
      fnctype, 
      tags, 
      timestamp, 
      nstime, 
      None, 
      None, 
      None, 
      None,
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
    fnctype: F,
    tags: Vec<&str>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
    solvable: Option<i64>,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>,
  ) -> Self {
    let mut f = Self {
      id: None,
      label: assemble_factor_name(varlbls.clone()),
      tags: Vec::new(),
      variableOrderSymbols_: Vec::new(),
      timestamp: Some(timestamp.unwrap_or(Utc::now())),
      nstime: Some(format!("{}", nstime.unwrap_or(0))),
      fnctype,
      solvable: Some(solvable.unwrap_or(0)),
      data: None,
      metadata: Some("e30=".to_string()),
      _version: Some(crate::SDK_VERSION.to_string()),
    };
    
    for vl in varlbls {
      f.variableOrderSymbols_.push(vl.to_string());
    }
    f.tags.push("FACTOR".to_string());
    for t in tags {
      if !t.eq("FACTOR") {
        f.tags.push(t.to_string());
      }
    }
    // default on create, also deser is different use-case    
    let fdata = FunctionData::new("FIXME", multihypo, nullhypo, inflation);
    // FIXME, should not be json'd so early: JuliaRobotics/DistributedFactorGraphs.jl#1118
    f.data = Some(fdata.to_json());
    
    return f;
  }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn post_add_factor<F>(
  nvafg: &NavAbilityDFG,
  factor: FactorDFG<F>,
) {

  // let request_body = GetVariable::build_query(
  //   get_variable::Variables {
  //       var_id: id.to_string(),
  //       fields_summary: true, // TODO simplify, since this must always be true
  //       fields_full,
  //   }
  // );

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
