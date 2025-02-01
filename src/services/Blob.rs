
use crate::{
    Utc,
    Uuid,
    GraphQLQuery,
    Response,
    Error,
    Sender,
    SDK_VERSION,
    NavAbilityClient,
    CreateDownload,
    create_download,
    CreateUpload,
    create_upload,
    CompleteUpload,
    complete_upload,
    DeleteBlob,
    delete_blob,
    check_deser,
    send_query_result,
    to_console_debug,
    to_console_error,
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn create_download_async(
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    store: Option<String>,
) -> Result<Response<create_download::ResponseData>, Box<dyn Error>> {

    let variables = create_download::Variables {
        blob_id: blob_id.to_string(),
        store: store.unwrap_or("default".to_string()).to_string(),
    };

    let request_body = CreateDownload::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<create_download::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn send_create_download(
    send_into: Sender<create_download::ResponseData>,
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    store: Option<String>
) {
    let resp = create_download_async(nvacl,blob_id,store).await;
    send_query_result::<create_download::ResponseData>(send_into,resp)
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn create_upload_async(
    nvacl: NavAbilityClient,
    // label: String,
    // blob_size: i64,
    blob_id: Uuid,
    parts: Option<i64>,
) -> Result<Response<create_upload::ResponseData>, Box<dyn Error>> {

    let variables = create_upload::Variables {
        // label: label.to_string(),
        blob_id: blob_id.to_string(),
        parts: parts.unwrap_or(1),
    };

    let request_body = CreateUpload::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<create_upload::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn complete_upload_async(
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    upload_id: String,
    etags: Vec<String>,
    // completed_upload: complete_upload::CompletedUploadInput,
) -> Result<Response<complete_upload::ResponseData>, Box<dyn Error>> {
    let mut parts: Vec<Option<complete_upload::CompletedUploadPartInput>> = vec![];
    for (i,et) in etags.iter().enumerate() {
        parts.push(
            Some(
                complete_upload::CompletedUploadPartInput {
                    part_number: (i + 1) as i64,
                    e_tag: Some(et.to_string()),
                }
            )
        )
    }

    let cupl = complete_upload::CompletedUploadInput {
        upload_id: upload_id.to_string(),
        parts
    };

    let variables = complete_upload::Variables {
        blob_id: blob_id.to_string(),
        completed_upload: cupl,
    };

    let request_body = CompleteUpload::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<complete_upload::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_delete_blob(
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    label: Option<&str>,
) -> Result<Response<delete_blob::ResponseData>, Box<dyn Error>> {
    
    let mut store = "default".to_owned();
    if let Some(lb) = label {
        store = lb.to_owned();
    }

    let variables = delete_blob::Variables {
        blob_id: blob_id.to_string(),
        label: Some(store)
    };
    let request_body = DeleteBlob::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<delete_blob::ResponseData>(
        req_res?.json().await
    )
}



#[cfg(target_arch = "wasm32")]
pub async fn create_upload_web(
    send_into: Sender<create_upload::ResponseData>, 
    client: &NavAbilityClient,
    name: &String,
    blob_size: i64,
    nparts: Option<i64>,
    blob_id: Option<Uuid>, // doenst work yet, leave None
) {
    let result = create_upload_async(
        client.clone(), 
        blob_id.expect("Must provide blob_id to create_upload_web"),
        nparts,
    ).await;
    send_query_result(send_into, result);
}
