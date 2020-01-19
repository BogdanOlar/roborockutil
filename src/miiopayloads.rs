use serde::{Serialize, Deserialize};
use std::str::{FromStr};

const METHOD_GET_STATUS_VAL:  &'static str = "get_status";

#[derive(Debug, Serialize)]
pub struct EmptyJsonObject {}

#[derive(Debug, Serialize)]
pub struct StatusCommand {
    pub id: u32,
    pub method: String,
    params: EmptyJsonObject
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub id: u32,
    pub result: Vec<StatusResponseResult>
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct StatusResponseResult {
    msg_ver: u32,
    msg_seq: u32,
    state: i32,
    battery: u32,
    clean_time: u32,
    clean_area: u32,
    error_code: i32,
    map_present: i32,
    in_cleaning: i32,
    in_returning: i32,
    in_fresh_state: i32,
    lab_status: i32,
    fan_power: i32,
    dnd_enabled: i32
}

impl StatusCommand {
    pub fn new(cmdid: u32) -> StatusCommand {
        return StatusCommand {
            id: cmdid,
            method: METHOD_GET_STATUS_VAL.to_string(),
            params: EmptyJsonObject{}
        }
    }
}

/// Return the length of the substring which contains the last `}` in the given string
///
/// This is useful because most `MiPacket`s coming from the RoboRock contain some junk bytes at the end of the
/// payload which causes the JSON parser to fail
///
/// # Arguments
/// `buf` - a string reference on which the search is performed
///
pub fn find_last_closing_bracket(buf: &str) -> usize {
    if let Some(pos) = buf.rfind('}') {
        return  pos + 1;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    const GET_STATUS_RESPONSE_STR: &str = "{\"result\":[{
                                            \"msg_ver\":2,
                                            \"msg_seq\":5,
                                            \"state\":8,
                                            \"battery\":100,
                                            \"clean_time\":2154,
                                            \"clean_area\":35692500,
                                            \"error_code\":0,
                                            \"map_present\":1,
                                            \"in_cleaning\":0,
                                            \"in_returning\":0,
                                            \"in_fresh_state\":1,
                                            \"lab_status\":1,
                                            \"fan_power\":60,
                                            \"dnd_enabled\":0
                                    }],
                                    \"id\":5
                                    }\u{0}\u{0}\u{0}";

    #[test]
    fn test_get_status_command() {
        let status_cmd = StatusCommand::new(1);
        let serialized = serde_json::to_string(&status_cmd).unwrap();
        assert_eq!(serialized, "{\"id\":1,\"method\":\"get_status\",\"params\":{}}");
    }

    #[test]
    fn test_get_status_response() {
        let end = find_last_closing_bracket(GET_STATUS_RESPONSE_STR);
        let status_response: StatusResponse = serde_json::from_str(&GET_STATUS_RESPONSE_STR[..end]).unwrap();
        let status_response_result = StatusResponseResult {
            msg_ver: 2,
            msg_seq: 5,
            state: 8,
            battery: 100,
            clean_time: 2154,
            clean_area: 35692500,
            error_code: 0,
            map_present: 1,
            in_cleaning: 0,
            in_returning: 0,
            in_fresh_state: 1,
            lab_status: 1,
            fan_power: 60,
            dnd_enabled: 0
        };
        let status_response_compare = StatusResponse {
            id: 5,
            result: vec!(status_response_result)
        };
        assert_eq!(status_response.id, status_response_compare.id);
        assert_eq!(status_response.result[0], status_response_compare.result[0]);
    }
}