
use crate::{
    check_query_response_data,
    get_variable::{
        self, 
        ppe_fields
    }, 
    list_variables, 
    parse_str_utc, 
    to_console_debug, 
    to_console_error, 
    Error, 
    GetId, 
    BlobEntry,
    GetVariable, 
    GraphQLQuery,
    ListVariables, 
    MeanMaxPPE, 
    PackedVariableNodeData, 
    Response, 
    Sender, 
    Utc, 
    Uuid, 
    VariableDFG, 
    SDK_VERSION
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    NavAbilityDFG,
    check_deser,
    send_query_result,
};


#[allow(non_snake_case)]
impl MeanMaxPPE {
    pub fn new(
        solveKey: &str,
        suggested: Vec<f64>,
        max: Vec<f64>,
        mean: Vec<f64>,
        _type: &str,
        _version: &str,
        createdTimestamp: Option<chrono::DateTime<Utc>>,
        lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
    ) -> Self {
        Self {
            id: None,
            solveKey: solveKey.to_owned(),
            suggested,
            max,
            mean,
            _type: _type.to_owned(),
            _version: _version.to_owned(),
            createdTimestamp,
            lastUpdatedTimestamp,
        }
    }

    pub fn from_gql(
        ppe: &ppe_fields
    ) -> Self {
        let mut ppesugg = Vec::new();
        if let Some(ps) = &ppe.suggested {
            for p in ps.iter() {
                if p.is_some() { ppesugg.push(p.unwrap().clone()); }
            }
        }
        let mut ppemax = Vec::new();
        if let Some(ps) = &ppe.max {
            for p in ps.iter() {
                if p.is_some() { ppemax.push(p.unwrap().clone()); }
            }
        }
        let mut ppemean = Vec::new();
        if let Some(ps) = &ppe.mean {
            for p in ps.iter() {
                if p.is_some() { ppemean.push(p.unwrap().clone()); }
            }
        }
        let mut ppe_struct = MeanMaxPPE {
            id: None,
            solveKey: ppe.solve_key.clone(),
            suggested: ppesugg,
            max: ppemax,
            mean: ppemean,
            _type: ppe.type_.clone(),
            _version: ppe.version.clone(),
            createdTimestamp: None,
            lastUpdatedTimestamp: None,
        };
        if let Ok(id) = Uuid::parse_str(
            &ppe.id
        ) {
            ppe_struct.id = Some(id);
        }
        if let Ok(dt) = parse_str_utc(
            ppe.created_timestamp.clone()
        ) {
            ppe_struct.createdTimestamp = Some(dt);
        }
        if let Ok(dt) = parse_str_utc(
            ppe.last_updated_timestamp.clone()
        ) {
            ppe_struct.lastUpdatedTimestamp = Some(dt);
        }
        return ppe_struct;
    }
}


#[allow(non_snake_case)]
impl VariableDFG {
    pub fn new(
        label: &str,
        variableType: &str,
        timestamp: Option<chrono::DateTime<Utc>>,
        nstime: Option<usize>,
    ) -> Self {
        let _ts = timestamp.unwrap_or(Utc::now());
        Self {
            id: None,
            label: label.to_owned(),
            tags: Vec::new(),
            timestamp: _ts,
            nstime: nstime.unwrap_or(0),
            ppes: Vec::new(),
            blobEntries: Vec::new(),
            variableType: variableType.to_string(),
            _version: SDK_VERSION.to_owned(),
            metadata: "".to_owned(),
            solvable: 1,
            solverData: Vec::new(),
        }
    }

    pub fn from_gql(
        vgql: &get_variable::GetVariableVariables
    ) -> Self {
        
        let timestamp = if let Ok(dt) = parse_str_utc(
            vgql.variable_summary_fields.timestamp.clone()
        ) {
            Some(dt)
        } else {
            None
        };
        let nstime = if let Ok(ns) = vgql.variable_summary_fields.nstime.parse::<usize>() {
            Some(ns)
        } else {
            None
        };

        let mut variable = Self::new(
            &vgql.variable_skeleton_fields.label.clone(),
            &vgql.variable_summary_fields.variable_type.clone(),
            timestamp,
            nstime,
        );

        // mutate additional information that is available
        if let Ok(id) = Uuid::parse_str(
            &vgql.variable_skeleton_fields.id
        ) {
            variable.id = Some(id);
        }
        // label
        let mut tags = Vec::new();
        for tag in vgql.variable_skeleton_fields.tags.iter() {
            if (&tag).is_some() {
                tags.push(tag.as_ref().unwrap().clone());
            }
        }
        variable.tags = tags;

        variable._version = vgql.variable_summary_fields.version.clone();
        variable.solvable = vgql.variable_full_fields.solvable as i32;
        if let Some(md) = vgql.variable_full_fields.metadata.clone() {
            variable.metadata = md;
        }

        let mut ppes = Vec::new();
        for ppe in &vgql.variable_summary_fields.ppes {
            ppes.push(MeanMaxPPE::from_gql(ppe));
        }
        variable.ppes = ppes;

        let mut bes = Vec::new();
        for be in &vgql.variable_summary_fields.blob_entries {
            bes.push(BlobEntry::from_gql(be));
        }
        variable.blobEntries = bes;

        let mut vnds = Vec::new();
        for vnd in &vgql.variable_full_fields.solver_data {
            vnds.push(PackedVariableNodeData::from_gql(vnd));
        }
        variable.solverData = vnds;

        return variable
    }
}


impl PackedVariableNodeData {
    pub fn new(
    ) -> Self {
        Self {
            id: None,
            vecval: Vec::new(),
            dimval: 0 as i32,
            vecbw: Vec::new(),
            dimbw: 0 as i32,
            BayesNetOutVertIDs: Vec::new(),
            dimIDs: Vec::new(),
            dims: 0 as i32,
            eliminated: false,
            BayesNetVertID: "".to_owned(),
            separator: Vec::new(),
            variableType: "".to_owned(),
            initialized: false,
            infoPerCoord: Vec::new(),
            ismargin: false,
            dontmargin: false,
            solveInProgress: 0 as i32,
            solvedCount: 0 as i32,
            solveKey: "".to_owned(),
            covar: Vec::new(),
            _version: SDK_VERSION.to_string(),
        }
    }

    pub fn from_gql(
        vndgql: &get_variable::solverdata_fields
    ) -> Self {
        return Self {
            id: Some(Uuid::parse_str(&vndgql.id).expect("failed to parse variable solver data id to uuid")),
            dimIDs: vndgql.dim_i_ds.clone().into_iter().map(|x| x as i32).collect(),
            infoPerCoord: vndgql.info_per_coord.clone(),
            BayesNetOutVertIDs: vndgql.bayes_net_out_vert_i_ds.clone().expect("PackedVariableNodeData to struct failed on BayesNetOutVertIDs"),
            separator: vndgql.separator.clone().unwrap_or(Vec::new()),
            vecval: vndgql.vecval.clone().expect("PackedVariableNodeData to struct failed on vecval"),
            vecbw: vndgql.vecbw.clone().expect("PackedVariableNodeData to struct failed on vecbw"),
            covar: vndgql.covar.clone().expect("PackedVariableNodeData to struct failed on covar"),
            dimval: vndgql.dimval as i32,
            dimbw: vndgql.dimbw as i32,
            dims: vndgql.dims as i32,
            solvedCount: vndgql.solved_count as i32,
            solveInProgress: vndgql.solve_in_progress as i32,
            initialized: vndgql.initialized,
            ismargin: vndgql.ismargin,
            dontmargin: vndgql.dontmargin,
            eliminated: vndgql.eliminated,
            BayesNetVertID: vndgql.bayes_net_vert_id.as_ref().unwrap_or(&"".to_string()).to_string(),
            variableType: vndgql.variable_type.to_string(),
            solveKey: vndgql.solve_key.to_string(), 
            _version: vndgql.version.to_string(),
        }
    }
}


// ===================== Queries =========================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_variable(
    nvafg: &NavAbilityDFG<'_>,
    label: &str,
    fields_summary: bool,
    fields_full: bool,
) -> Result<Response<get_variable::ResponseData>, Box<dyn Error>> {
    let id = nvafg.fg.getId(label); 
    let request_body = GetVariable::build_query(
        get_variable::Variables {
            var_id: id.to_string(),
            fields_summary,
            fields_full,
        }
    );

    let req_res = nvafg.client.client
    .post(&nvafg.client.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_variable::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn getVariable(
    nvafg: &NavAbilityDFG<'_>,
    label: &str,
    fields_summary: bool,
    fields_full: bool,
) -> Option<VariableDFG> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let response_body = rt.block_on(async { 
        fetch_variable(
            nvafg,
            label,
            fields_summary,
            fields_full,
        ).await
    });

    let variable_data = check_query_response_data::<
        get_variable::ResponseData,
        get_variable::ResponseData
    >(response_body, |s| {s});

    match variable_data {
        Ok(vdata) => {
            if 0 < vdata.variables.len() {
                // FIXME return the entire list of variables
                return Some(VariableDFG::from_gql(&vdata.variables[0]))
            }
        },
        Err(e) => {
            to_console_error(&format!("NvaSDK.rs error during getVariable: {:?}", e));
        }
    }
    return None
}


// TODO get better function signature
// #[allow(non_snake_case)]
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_variables(
    nvafg: &NavAbilityDFG<'_>,
) -> Result<Response<list_variables::ResponseData>, Box<dyn Error>> {
    let id = nvafg.fg.getId(""); 
    let request_body = ListVariables::build_query(
        list_variables::Variables {
            fg_id: id.to_string(),
            solvable_gt: None,
            solvable_gte: None,
            solvable_in: None,
            solvable_lt: None,
            solvable_lte: None,
            tags_includes: None,
        }
    );

    let req_res = nvafg.client.client
    .post(&nvafg.client.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<list_variables::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn send_list_variables(
    send_into: Sender<list_variables::ResponseData>,
    nvafg: &NavAbilityDFG<'_>,
) {
    let resp = fetch_list_variables(nvafg).await;
    let _ = send_query_result::<
        list_variables::ResponseData,
        list_variables::ResponseData
    >(send_into, resp, |s| {s});
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn listVariables(
    nvafg: &NavAbilityDFG<'_>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let response_body = rt.block_on(async { 
        fetch_list_variables(
            nvafg,
        ).await
    });

    return check_query_response_data::<
        list_variables::ResponseData,
        Vec<String>
    >(response_body, |s| {s.list_variables});
}
