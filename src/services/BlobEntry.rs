
use crate::{
    get_variable, 
    parse_str_utc, 
    send_query_result, 
    to_console_debug, 
    to_console_error, 
    update_blobentry_metadata, 
    BlobEntry, 
    Error, 
    Response, 
    UpdateBlobentryMetadata, 
    Utc, 
    Uuid, 
    SDK_VERSION,
    chrono::ParseError,
};

#[macro_use]
use crate::BlobEntryFieldsAccessors;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    check_deser, 
    DeleteBlobEntry,
    delete_blob_entry,
    GetBlobEntry, 
    get_blob_entry, 
    SameBlobEntryFields,
    GraphQLQuery, 
    NavAbilityClient,
};

use crate::BlobEntry_accessors;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use get_blob_entry::blobEntry_fields as GB_BlobEntryFields;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use get_variable::blobEntry_fields as GV_BlobEntryFields;

// duplication in blobEntry_fields GQL fragments in get_blob_entry and  get_variable
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_accessors!(GB_BlobEntryFields);
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_accessors!(GV_BlobEntryFields);



impl BlobEntry {
    pub fn new() -> Self {
        let mut be = BlobEntry::default();
        be.blobId = Uuid::new_v4();
        be.blobstore = "default".to_string();
        be.origin = "NvaSDK.rs".to_string();
        be.metadata = "e30=".to_string();
        be.createdTimestamp = Some(Utc::now());
        be.lastUpdatedTimestamp = be.createdTimestamp.clone();
        be._type = "BlobEntry".to_string(); // for self assemply typed usage elsewhere
        be._version = SDK_VERSION.to_string(); // FIXME dont hardcode, pull from common source
        return be
    }

    #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
    pub fn from_gql(
        begql: &impl BlobEntryFieldsAccessors,
    ) -> Self {
        let mut be = BlobEntry::default();
        be.id = begql.id();
        be.blobId = begql.blobId();
        be.originId = begql.originId();
        be.label = begql.label();
        be.blobstore = begql.blobstore();
        be.hash = begql.hash();
        be.origin = begql.origin();
        be.size = begql.size();
        be.description = begql.description();
        be.mimeType = begql.mimeType();
        be.metadata = begql.metadata();
        be.timestamp = begql.timestamp().unwrap_or(Utc::now());
        be.createdTimestamp = begql.createdTimestamp();
        be.lastUpdatedTimestamp = begql.lastUpdatedTimestamp();        
        be._type = begql._type();
        be._version = begql._version();

        return be;
    }

    #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
    pub fn try_from_receiver(
        rx: &std::sync::mpsc::Receiver<get_blob_entry::ResponseData>
    ) -> Option<Self> {
        
        match rx.try_recv() {
            Ok(gqle) => {
                // FIXME return Vec<BlobEntry>
                let gety = &gqle.blob_entries[0];
                return Some(Self::from_gql(gety));
            }
            Err(_e) => {
                // to_console_debug(&"BlobEntry::try_from_receive nothing in channel");
            }
        }

        return None
    }
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_delete_blobentry(
    nvacl: NavAbilityClient,
    id: Uuid,
) -> Result<Response<delete_blob_entry::ResponseData>, Box<dyn Error>> {
    
    let variables = delete_blob_entry::Variables {
        id: id.to_string(),
    };
    let request_body = DeleteBlobEntry::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<delete_blob_entry::ResponseData>(
        req_res?.json().await
    )
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn update_blobentry_metadata_async(
    nvacl: NavAbilityClient,
    id: &Uuid,
    metadata_b64: &str
) -> Result<Response<update_blobentry_metadata::ResponseData>,Box<dyn Error>> {

    let variables = update_blobentry_metadata::Variables {
        id: id.to_string(),
        metadata: metadata_b64.to_string(),
    };

    let request_body = UpdateBlobentryMetadata::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<update_blobentry_metadata::ResponseData>(
        req_res?.json().await
    )
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_blob_entry(
    nvacl: NavAbilityClient,
    id: Uuid
) -> Result<Response<get_blob_entry::ResponseData>, Box<dyn Error>> {

    let variables = get_blob_entry::Variables {
        entry_id: id.to_string(),
    };

    let request_body = GetBlobEntry::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_blob_entry::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn send_blob_entry(
    send_into: std::sync::mpsc::Sender<get_blob_entry::ResponseData>,
    nvacl: NavAbilityClient,
    id: Uuid
) {
    let resp = fetch_blob_entry(nvacl, id).await;
    let _ = send_query_result::<
        get_blob_entry::ResponseData,
        get_blob_entry::ResponseData
    >(send_into, resp, |s| {s});
}



// =============== FUTURE IDEAS ==============


// impl add_blob_entries::BlobEntryCreateInput {
//     pub fn new(
//         org_id: Uuid,
//         entry: &BlobEntry,
//         agent_label: String,
//     ) -> Self {

//         let name = format!("{}{}",&agent_label,&entry.label).to_string();
//         let id = Uuid::new_v5(&org_id, name.as_bytes());

//         let node = add_blob_entries::AgentWhere::new_agent(Some(agent_label.to_string()));

//         let agent_connect_where = add_blob_entries::AgentConnectWhere {
//             node: Box::new(node),
//         };

//         let agent = add_blob_entries::BlobEntryParentAgentFieldInput {
//             create: None,
//             connect_or_create: None,
//             connect: Some(add_blob_entries::BlobEntryParentAgentConnectFieldInput {
//                 connect: None,
//                 where_: Some(agent_connect_where),
//             })
//         };

//         let parent = add_blob_entries::BlobEntryParentCreateInput {
//             agent: Some(agent),
//             model: None,
//             factorgraph: None,
//             variable: None,
//             factor: None
//         };
            

//         Self {
//             id: id.to_string(),
//             origin_id: Some(entry.blobId.to_string()),
//             blob_id: entry.blobId.to_string(),
//             label: entry.label.to_string(),
//             blobstore: Some(entry.blobstore.to_string()),
//             origin: Some(entry.origin.to_string()),
//             description: Some(entry.description.to_string()),
//             mime_type: Some(entry.mimeType.to_string()),
//             hash: Some(entry.hash.to_string()),
//             size: entry.size,
//             metadata: Some(entry.metadata.to_string()),
//             timestamp: Some(entry.timestamp.to_string()),
//             // type_: entry._type.to_string(),
//             version: entry._version.to_string(),
//             parent: Some(parent),
//         }
//     }
// }
// 
// impl add_blob_entries::AgentWhere {
//     pub fn new_agent(
//         agent_label: Option<String>
//     ) -> Self {
//         Self {
//             id: None,
//             id_in: None,
//             id_contains: None,
//             id_starts_with: None,
//             id_ends_with: None,
//             org: Box::new(None),
//             org_aggregate: Box::new(None),
//             org_connection: Box::new(None),
//             org_connection_not: Box::new(None),
//             org_not: Box::new(None),
//             tags: None,
//             tags_includes: None,
//             version_ends_with: None,
//             version_starts_with: None,
//             version: None,
//             version_contains: None,
//             version_in: None,
//             and: Box::new(None),
//             not: Box::new(None),
//             or: Box::new(None),
//             blob_entries_aggregate: Box::new(None),
//             blob_entries_all: Box::new(None),
//             blob_entries_connection_all: Box::new(None),
//             blob_entries_connection_none: Box::new(None),
//             blob_entries_connection_single: Box::new(None),
//             blob_entries_connection_some: Box::new(None),
//             blob_entries_none: Box::new(None),
//             blob_entries_single: Box::new(None),
//             blob_entries_some: Box::new(None),
//             created_timestamp: None,
//             created_timestamp_gt: None,
//             created_timestamp_gte: None,
//             created_timestamp_in: None,
//             created_timestamp_lt: None,
//             created_timestamp_lte: None,
//             last_updated_timestamp: None,
//             last_updated_timestamp_gt: None,
//             last_updated_timestamp_gte: None,
//             last_updated_timestamp_lt: None,
//             last_updated_timestamp_lte: None,
//             last_updated_timestamp_in: None,
//             description: None,
//             description_contains: None,
//             description_ends_with: None,
//             description_in: None,
//             description_starts_with: None,
//             fgs_aggregate: Box::new(None),
//             fgs_all: Box::new(None),
//             fgs_single: Box::new(None),
//             fgs_connection_all: Box::new(None),
//             fgs_connection_single: Box::new(None),
//             fgs_connection_none: Box::new(None),
//             fgs_connection_some: Box::new(None),
//             fgs_none: Box::new(None),
//             fgs_some: Box::new(None),
//             label: agent_label,
//             label_in: None,
//             label_contains: None,
//             label_starts_with: None,
//             label_ends_with: None,
//             metadata: None,
//             metadata_in: None,
//             models_aggregate: Box::new(None),
//             models_all: Box::new(None),
//             models_none: Box::new(None),
//             models_some: Box::new(None),
//             models_single: Box::new(None),
//             models_connection_all: Box::new(None),
//             models_connection_none: Box::new(None),
//             models_connection_some: Box::new(None),
//             models_connection_single: Box::new(None),
//         }
//     }
// }




// ================== DEPRECATED ===================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl BlobEntry {
    pub fn same_gql(
        sgql: impl SameBlobEntryFields,
    ) -> get_blob_entry::blobEntry_fields {
        return sgql.to_gql_blobentry();
    }

    // get_blob_entry::blobEntry_fields
    pub fn from_gql2(
        gety: &get_blob_entry::blobEntry_fields
    ) -> Self {
        let mut be = BlobEntry::default();
        be.id = Some(Uuid::parse_str(&gety.id).expect("failed to parse entry id to uuid"));
        be.blobId = Uuid::parse_str(&gety.blob_id).expect("failed to parse entry blob_id to uuid");
        be.label = gety.label.to_string();
        if let Some(blobstore) = &gety.blobstore {
            be.blobstore = blobstore.to_string();
        }
        if let Some(origin) = &gety.origin {
            be.origin = origin.to_string();
        }
        if let Some(mime) = &gety.mime_type {
            be.mimeType = mime.to_string();
        }
        if let Some(description) = &gety.description {
            be.description = description.to_string();
        }
        if let Some(hash) = &gety.hash {
            be.hash = hash.to_string();
        }
        if let Some(metadata) = &gety.metadata {
            be.metadata = metadata.to_string();
        }
        if let Some(size) = &gety.size {
            be.size = Some((*size).parse::<i64>().unwrap());
        }
        if let Some(timestamp) = &gety.timestamp {
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                be.timestamp = tms;
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        {
            let timestamp = &gety.created_timestamp;
            // to_console_debug(&format!("BlobEntry from rx timestamp string {}",&timestamp));
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                be.createdTimestamp = Some(tms);
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        {
            let timestamp = &gety.last_updated_timestamp;
            // to_console_debug(&format!("BlobEntry from rx timestamp string {}",&timestamp));
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(
                timestamp.clone()
            ) {
                be.lastUpdatedTimestamp = Some(tms);
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        if let Some(_type) = &gety.type_ {
            be._type = _type.to_string();
        }
        be._version = gety.version.to_string();

        return be;
    }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl SameBlobEntryFields for get_blob_entry::blobEntry_fields {
    fn to_gql_blobentry(self) -> get_blob_entry::blobEntry_fields {
        return self;
    }
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl SameBlobEntryFields for get_variable::blobEntry_fields {
    fn to_gql_blobentry(
        self
    ) -> get_blob_entry::blobEntry_fields {
        return get_blob_entry::blobEntry_fields {
            id: self.id.clone(),
            blob_id: self.blob_id.clone(),
            origin_id: self.origin_id.clone(),
            label: self.label.clone(),
            blobstore: self.blobstore.clone(),
            hash: self.hash.clone(),
            origin: self.origin.clone(),
            size: self.size.clone(),
            description: self.description.clone(),
            mime_type: self.mime_type.clone(),
            metadata: self.metadata.clone(),
            timestamp: self.timestamp.clone(),
            created_timestamp: self.created_timestamp.clone(),
            last_updated_timestamp: self.last_updated_timestamp.clone(),
            version: self.version.clone(),
            type_: self.type_.clone(),
        }
    }
}