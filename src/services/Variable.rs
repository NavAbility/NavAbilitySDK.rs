
use crate::{
    Utc,
    GraphQLQuery,
    Response,
    Error,
    GetId,
    NavAbilityDFG,
    VariableDFG,
    PackedVariableNodeData,
    get_variable,
    GetVariable,
    // list_variables,
    // ListVariables,
    check_deser,
    to_console_debug,
    to_console_error,
    SDK_VERSION,
};


#[allow(non_snake_case)]
impl VariableDFG<'_> {
    pub fn new(
        label: &str,
        variableType: &str,
        timestamp: Option<chrono::DateTime<Utc>>,
        nstime: Option<String>,
    ) -> Self {
        let _ts = timestamp.unwrap_or(Utc::now());
        Self {
            id: None,
            label: label.to_owned(),
            tags: Vec::new(),
            timestamp: _ts,
            nstime: nstime.unwrap_or("".to_owned()),
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

        let mut variable = Self::new(
            &vgql.variable_skeleton_fields.label.clone(),
            &vgql.variable_summary_fields.variable_type.clone(),
            None,
            None,
        );

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
}


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
pub fn getVariable<'a>(
    nvafg: &NavAbilityDFG<'_>,
    label: &'a str,
    fields_summary: bool,
    fields_full: bool,
) -> Option<VariableDFG<'a>> {
    use crate::check_query_response_data;

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

    let variable_data = check_query_response_data::<get_variable::ResponseData>(response_body);

    match variable_data {
        Ok(vdata) => {
            if 0 < vdata.variables.len() {
                return Some(VariableDFG::from_gql(&vdata.variables[0]))
            }
        },
        Err(e) => {
            to_console_error(&format!("NvaSDK.rs error during getVariable: {:?}", e));
        }
    }
    return None
}


// // TODO get better function signature
// // #[allow(non_snake_case)]
// #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
// pub async fn fetch_list_variables(
//     nvafg: &NavAbilityDFG<'_>,
// ) -> Result<Response<list_variables::ResponseData>, Box<dyn Error>> {
//     let id = nvafg.fg.getId(""); 
//     let request_body = ListVariables::build_query(
//         list_variables::Variables {
//             fg_id: id.to_string(),
//         }
//     );

//     let req_res = nvafg.client.client
//     .post(&nvafg.client.apiurl)
//     .json(&request_body)
//     .send().await;

//     if let Err(ref re) = req_res {
//         to_console_error(&format!("API request error: {:?}", re));
//     }

//     return check_deser::<list_variables::ResponseData>(
//         req_res?.json().await
//     )
// }
