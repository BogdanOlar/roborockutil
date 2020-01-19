use serde::{Serialize, Deserialize};
use std::str::{FromStr};

//const KEY_ID:  &'static str = "id";

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub id: u32,
    pub result: Vec<StatusResponseResult>
}

#[derive(Debug, Deserialize)]
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
    const GET_STATUS_RESPONSE_STR:&str = "{\"result\":[{
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
    fn test_json() {
        let end = find_last_closing_bracket(GET_STATUS_RESPONSE_STR);
        println!("end: {}, len: {}", end, GET_STATUS_RESPONSE_STR.len());
        let t: Value = serde_json::from_str(&GET_STATUS_RESPONSE_STR[..end]).unwrap();
        println!("{:?}", t);
        let u: StatusResponse = serde_json::from_str(&GET_STATUS_RESPONSE_STR[..end]).unwrap();
        println!("{:?}", u);
    }
}