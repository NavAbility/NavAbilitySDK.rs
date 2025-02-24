
use uuid::Uuid;

use chrono::{
  DateTime,
  Utc
};

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};


use crate::{
  FullNormal,
  PriorPoint2,
  PriorPoint3,
  PriorPose2,
  PriorPose3,
  Point2Point2,
  Point3Point3,
  Pose2Pose2,
  Pose3Pose3,
};



#[allow(non_snake_case)]
impl<'a> PriorPoint2<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}



#[allow(non_snake_case)]
impl<'a> PriorPoint3<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> PriorPose2<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> PriorPose3<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> Point2Point2<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> Point3Point3<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> Pose2Pose2<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}


#[allow(non_snake_case)]
impl<'a> Pose3Pose3<FullNormal<'a>> {
  pub fn new(
    Z: FullNormal<'a>
  ) -> Self {
    Self {
      Z: Z
    }
  }
}




// =======================================================


use crate::{
  FunctionData,
  FactorDFG
};


fn assemble_factor_name(
  ovlb: Vec<&str>
) -> String {
  let mut flb = "".to_string();
  for o in ovlb {
    flb += o;
  } 
  flb += "_";
  flb += &(Uuid::new_v4().to_string()[0..4]);
  
  return flb;
}


fn get_fnc_name(
  fnc: &str
) -> String {
  let parts = fnc.split(".");
  let mut t= "";
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
    inflation: Option<f64>
  ) -> Self {
    let mut fd = Self::default();
    fd.fnc = fnc.to_owned();
    fd.nullhypo = nullhypo.unwrap_or(0.0);
    fd.multihypo = multihypo.unwrap_or(Vec::new());
    fd.inflation = inflation.unwrap_or(3.0);
    return fd;
  }

  pub fn to_json(
    &self,
  ) -> String {
    serde_json::to_string(self).unwrap().to_string()
  }

  pub fn to_string_b64(
    &self,
  ) -> String { 
    general_purpose::STANDARD.encode(self.to_json().to_string())
  }
}



impl FactorDFG {
  pub fn new_more(
    varlbls: Vec<&str>,
    fnctype: &str,
    tags: Vec<&str>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
    solvable: Option<i64>,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>
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
    f.nstime = Some(format!("{}",nstime.unwrap_or(0)));
    f.solvable = Some(solvable.unwrap_or(0));

    let fdata = FunctionData::new(
      &f.fnctype.to_string(),
      multihypo,
      nullhypo,
      inflation,
    );
    f.data = Some(fdata.to_string_b64());
    
    return f;
  }
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