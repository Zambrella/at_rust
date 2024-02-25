use at_records::{at_key::AtKey, at_record::AtValue};

use super::prelude::*;

pub struct UpdateVerbInputs<'a> {
    /// The AtKey of the key-value pair to be looked up.
    at_key: &'a AtKey,

    /// The value to be updated.
    value: AtValue,

    /// Time to live in milliseconds
    ttl: Option<usize>,

    /// Time to birth in milliseconds
    ttb: Option<usize>,

    /// Time to refresh in milliseconds. ttr > -1 is a valid value which indicates that the user with whom the key has been shared can keep it forever and the value for this key won't change forever.
    ttr: Option<usize>,

    /// Indicates if a cached key needs to be deleted when the atSign user who has originally shared it deletes it.
    ccd: Option<bool>,
}

/// The update verb should be used to perform create/update operations on the atServer.
/// The update verb requires the owner of the atServer to authenticate themself to the atServer using from and cram verbs.
pub struct UpdateVerb;

impl<'a> Verb<'a> for UpdateVerb {
    type Inputs = UpdateVerbInputs<'a>;
    type Output = String;

    fn execute(tls_client: &mut TlsClient, input: Self::Inputs) -> Result<Self::Output> {
        let mut string_buf = String::from("update:");

        if let Some(ttl) = input.ttl {
            string_buf.push_str(format!("ttl:{},", ttl).as_str());
        }

        if let Some(ttb) = input.ttb {
            string_buf.push_str(format!("ttb:{},", ttb).as_str());
        }

        if let Some(ttr) = input.ttr {
            string_buf.push_str(format!("ttr:{},", ttr).as_str());
        }

        if let Some(ccd) = input.ccd {
            string_buf.push_str(format!("ccd:{},", ccd).as_str());
        }

        // Private and internal are not supported. Throw error?
        let visibility = match &input.at_key.visibility_scope {
            at_records::at_key::Visibility::Private => String::from(""),
            at_records::at_key::Visibility::Internal => String::from(""),
            at_records::at_key::Visibility::Public => String::from("public:"),
            at_records::at_key::Visibility::Shared(shared_with) => {
                format!("{}:", shared_with.get_at_sign_with_prefix())
            }
        };

        let formatted_at_key = format!(
            "{visibility}{record_id}.{namespace}{owner}",
            visibility = visibility,
            record_id = &input.at_key.record_id,
            namespace = input.at_key.namespace.as_ref().unwrap_or(&String::from("")),
            owner = &input.at_key.owner.get_at_sign_with_prefix()
        );
        string_buf.push_str(formatted_at_key.as_str());

        let value = match input.value {
            AtValue::Text(text) => text,
            AtValue::Binary(data) => todo!(),
        };

        string_buf.push_str(format!(" {}", value).as_str());

        tls_client.send_data(string_buf)?;

        let response_data = tls_client.read_data()?;
        let response_string = Self::parse_server_response(&response_data, "data")?;

        Ok(response_string)
    }
}
